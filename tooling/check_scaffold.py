#!/usr/bin/env python3
"""Read-only repository contract checks. Never run dependency commands or publish."""
import argparse
import json
from pathlib import Path, PurePosixPath
import re

STATES = {'ready', 'in_progress', 'blocked', 'needs_input', 'completed', 'cancelled'}
TERMINAL = {'completed', 'cancelled'}


def block(path):
    found = re.findall(r'^```json\n(.*?)^```\s*$', path.read_text(), re.M | re.S)
    if len(found) != 1:
        raise ValueError(f'{path}: expected exactly one JSON contract block')
    return json.loads(found[0])


def safe_path(path):
    return (isinstance(path, str) and bool(path) and not path.startswith('/')
            and '\\' not in path and all(p not in {'', '.', '..'} for p in path.rstrip('/').split('/')))


def owns(owned, path):
    return any(path == p or p.endswith('/') and path.startswith(p) for p in owned)


def overlap(a, b):
    return a.rstrip('/') == b.rstrip('/') or a.endswith('/') and b.startswith(a) or b.endswith('/') and a.startswith(b)


def validate(root, selected=None):
    """Return errors; this function has no writes or subprocess calls."""
    root = Path(root)
    errors = []
    try:
        roles = block(root / '.sys/black-hole/role-map.md')
        if not isinstance(roles, dict) or not roles:
            raise ValueError('role map must be a nonempty object')
        if selected and selected not in roles:
            errors.append(f'unknown role: {selected}')
        for role, data in roles.items():
            if not re.fullmatch(r'[a-z][a-z0-9-]*', role):
                errors.append(f'invalid role ID: {role}')
            paths = data['owned']
            if not isinstance(paths, list) or not paths:
                raise ValueError(f'{role}: owned must be a nonempty list')
            for p in paths:
                if not safe_path(p):
                    errors.append(f'{role}: unsafe owned path {p}')
            expected = [f'.sys/plans/{role}/', f'.sys/backlogs/{role}.md', f'.sys/memory/{role}.md', f'docs/status/{role.upper()}.md']
            for p in expected:
                if not owns(paths, p):
                    errors.append(f'{role}: missing owned state path {p}')
            for locked in ['VISION.md', 'AGENTS.md', '.sys/black-hole/', 'docs/prompts/', 'docs/decisions/', 'spec/']:
                if any(overlap(p, locked) for p in paths):
                    errors.append(f'{role}: owns locked input {locked}')
        for a in roles:
            for b in roles:
                if a >= b:
                    continue
                for p in roles[a]['owned']:
                    for q in roles[b]['owned']:
                        if overlap(p, q):
                            errors.append(f'ownership overlap: {a}:{p} and {b}:{q}')
        for p in [root/'VISION.md', root/'AGENTS.md', *list((root/'.sys').rglob('*.md')), *list((root/'docs/prompts').glob('*.md'))]:
            if '{{' in p.read_text() or '}}' in p.read_text():
                errors.append(f'unresolved template delimiter: {p.relative_to(root)}')
        entries = {}
        plans = {}
        seen_ids = set()
        for role, data in roles.items():
            for prompt in ['planning', 'execution']:
                text = (root/f'docs/prompts/{prompt}-{role}.md').read_text()
                for required in ['`VISION.md`', f'`.sys/plans/{role}/`', f'`.sys/backlogs/{role}.md`', f'`.sys/memory/{role}.md`', f'`docs/status/{role.upper()}.md`']:
                    if required not in text:
                        errors.append(f'{role}: {prompt} missing {required}')
                for p in data['owned']:
                    if f'`{p}`' not in text:
                        errors.append(f'{role}: {prompt} missing owned path {p}')
            for p in [f'.sys/memory/{role}.md', f'docs/status/{role.upper()}.md']:
                if not (root/p).is_file():
                    errors.append(f'missing {p}')
            ledger = block(root/f'.sys/backlogs/{role}.md')
            if not isinstance(ledger, list):
                raise ValueError(f'{role}: backlog must be a list')
            active = [e for e in ledger if e.get('state') not in TERMINAL]
            if len(active) > 1:
                errors.append(f'{role}: stacked nonterminal work')
            for e in ledger:
                if e['id'] in seen_ids:
                    errors.append(f'duplicate work-order ID: {e["id"]}')
                seen_ids.add(e['id'])
                state = e['state']
                if state not in STATES:
                    errors.append(f'{e["id"]}: invalid state {state}')
                if not isinstance(e['attempt'], int) or isinstance(e['attempt'], bool) or e['attempt'] < 0:
                    errors.append(f'{e["id"]}: invalid attempt')
                if state == 'in_progress' and (e['claim'] == 'none' or not e['claim'] or e['attempt'] < 1):
                    errors.append(f'{e["id"]}: missing active claim')
                if state != 'in_progress' and e['claim'] != 'none':
                    errors.append(f'{e["id"]}: inactive claim must be none')
                if state in {'blocked', 'needs_input'} and e['result'] in {'', 'none'}:
                    errors.append(f'{e["id"]}: missing recovery evidence/question')
                path = e['plan']
                if not safe_path(path) or not path.startswith(f'.sys/plans/{role}/'):
                    errors.append(f'{e["id"]}: plan outside owner namespace')
                    continue
                if path in entries:
                    errors.append(f'duplicate plan entry: {path}')
                entries[path] = (role, e)
                plan = block(root/path)
                plans[path] = plan
                if plan['id'] != e['id'] or plan['role'] != role:
                    errors.append(f'{path}: plan ownership/identity mismatch')
                writes = plan['create'] + plan['modify']
                for p in writes:
                    if not safe_path(p) or not owns(data['product'], p):
                        errors.append(f'{path}: write outside product ownership: {p}')
                    elif (root/p).is_symlink() or not (root/p).resolve().is_relative_to(root.resolve()):
                        errors.append(f'{path}: write resolves outside repository: {p}')
                if len(writes) != len(set(writes)):
                    errors.append(f'{path}: duplicate write inventory')
                if set(writes) & set(plan['read_only']):
                    errors.append(f'{path}: read-only/write conflict')
                for p in plan['read_only']:
                    if not safe_path(p) or not (root/p).exists():
                        errors.append(f'{path}: missing read-only evidence: {p}')
                text = (root/path).read_text()
                for n, name in enumerate(['Context & Goal', 'File Inventory', 'Implementation Spec', 'Test Plan', 'Constraints', 'Dependencies', 'Result'], 1):
                    if f'## {n}. {name}' not in text:
                        errors.append(f'{path}: missing section {n}')
            discovered = {str(p.relative_to(root)) for p in (root/f'.sys/plans/{role}').glob('*.md')}
            known = {p for p, (r, _) in entries.items() if r == role}
            for orphan in discovered-known:
                errors.append(f'unindexed plan: {orphan}')
        graph = {}
        for path, plan in plans.items():
            graph[path] = []
            for dep in plan['dependencies']:
                owner, target = dep['role'], dep['plan']
                if owner not in roles or target not in entries:
                    errors.append(f'{path}: stale/missing dependency {owner}:{target}')
                    continue
                if entries[target][0] != owner:
                    errors.append(f'{path}: dependency owner mismatch {target}')
                if not isinstance(dep['unblock'], str) or not dep['unblock'].strip():
                    errors.append(f'{path}: missing executable unblock condition')
                if entries[path][1]['state'] in {'ready', 'in_progress'} and entries[target][1]['state'] != 'completed':
                    errors.append(f'{path}: executable work has unresolved dependency {target}')
                graph[path].append(target)
        def cycle(node, visiting, done):
            if node in visiting:
                return True
            if node in done:
                return False
            visiting.add(node)
            if any(cycle(n, visiting, done) for n in graph.get(node, [])):
                return True
            visiting.remove(node)
            done.add(node)
            return False
        done = set()
        if any(cycle(node, set(), done) for node in graph):
            errors.append('dependency cycle; needs an explicit human scope/ownership decision')
    except (OSError, ValueError, KeyError, TypeError) as exc:
        errors.append(f'malformed scaffold: {exc}')
    return errors


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument('--role', help='Check a role exists; global invariants are always checked')
    args = parser.parse_args()
    failures = validate(args.root, args.role)
    if failures:
        print('\n'.join(f'ERROR: {e}' for e in failures))
        raise SystemExit(1)
    print('PASS: scaffold contracts; no product parity or runtime convergence asserted')
