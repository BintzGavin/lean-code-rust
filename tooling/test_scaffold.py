import hashlib
import json
from pathlib import Path
import re
import shutil
import tempfile
import unittest

from check_scaffold import block, validate


class ScaffoldContracts(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)/'repo'
        source = Path(__file__).resolve().parents[1]
        self.source = source
        shutil.copytree(source, self.root, ignore=shutil.ignore_patterns('.git', 'target', '__pycache__'))
        # Contract fixtures must not inherit live queued work or dependencies.
        initial_plan = '.sys/plans/editor-core/editor-core-001-document-roundtrip.md'
        for p in (self.root/'.sys/plans').rglob('*.md'):
            if str(p.relative_to(self.root)) != initial_plan:
                p.unlink()
        def retain_initial(data):
            data[:] = [e for e in data if e['plan'] == initial_plan]
        for p in (self.root/'.sys/backlogs').glob('*.md'):
            self.edit(p.relative_to(self.root), retain_initial)
        self.edit(initial_plan, lambda d: d.update(dependencies=[]))

    def tearDown(self):
        self.temp.cleanup()

    def edit(self, path, change):
        p = self.root/path
        data = block(p)
        change(data)
        p.write_text(re.sub(r'^```json\n.*?^```\s*$', lambda _: '```json\n'+json.dumps(data, indent=2)+'\n```\n', p.read_text(), flags=re.M|re.S))

    def expect(self, text):
        self.assertTrue(any(text in e for e in validate(self.root)), validate(self.root))

    def fingerprint(self):
        return {str(p.relative_to(self.root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in self.root.rglob('*') if p.is_file()}

    def test_current_repository_scaffold_is_valid(self):
        self.assertEqual([], validate(self.source))

    def test_scaffold_checker_is_read_only(self):
        before = self.fingerprint()
        self.assertEqual([], validate(self.root))
        self.assertEqual(before, self.fingerprint())

    def test_empty_idle_backlogs_are_valid_without_publication_gate(self):
        for p in (self.root/'.sys/plans').rglob('*.md'):
            p.unlink()
        for p in (self.root/'.sys/backlogs').glob('*.md'):
            self.edit(p.relative_to(self.root), lambda data: data.clear())
        before = self.fingerprint()
        self.assertEqual([], validate(self.root))
        self.assertEqual(before, self.fingerprint())

    def test_directory_ownership_overlap(self):
        self.edit('.sys/black-hole/role-map.md', lambda d: d['desktop']['owned'].append('crates/'))
        self.expect('ownership overlap')

    def test_cross_owner_plan_rejected(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['create'].append('crates/extensions/host.rs'))
        self.expect('write outside product ownership')

    def test_stale_dependency_rejected(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['dependencies'].append({'role':'extensions','plan':'.sys/plans/extensions/missing.md','unblock':'cargo test'}))
        self.expect('stale/missing dependency')

    def test_plan_owner_mismatch(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d.update(role='desktop'))
        self.expect('ownership/identity mismatch')

    def test_claim_required(self):
        self.edit('.sys/backlogs/editor-core.md', lambda d: d[0].update(state='in_progress',claim='none'))
        self.expect('missing active claim')

    def test_blocked_requires_evidence(self):
        self.edit('.sys/backlogs/editor-core.md', lambda d: d[0].update(state='blocked',claim='none',result='none'))
        self.expect('missing recovery evidence')

    def test_blocked_with_recovery_evidence_is_preserved(self):
        self.edit('.sys/backlogs/editor-core.md', lambda d: d[0].update(state='blocked',claim='none',result='Resource limit: resume after memory_pressure reports at least 40% free and rerun the named core checks.'))
        self.assertEqual([], validate(self.root))

    def test_stacked_nonterminal_work_is_rejected(self):
        source = self.root/'.sys/plans/editor-core/editor-core-001-document-roundtrip.md'
        target = source.with_name('editor-core-second.md')
        target.write_text(source.read_text())
        self.edit(target.relative_to(self.root), lambda d: d.update(id='EDITOR-CORE-SECOND'))
        def stack(data):
            data[0].update(state='ready',claim='none')
            data.append(dict(data[0],id='EDITOR-CORE-SECOND',plan=str(target.relative_to(self.root))))
        self.edit('.sys/backlogs/editor-core.md',stack)
        self.expect('stacked nonterminal work')

    def test_readonly_write_conflict(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['read_only'].append(d['create'][0]))
        self.expect('read-only/write conflict')

    def test_unindexed_plan(self):
        shutil.copy(self.root/'.sys/plans/editor-core/editor-core-001-document-roundtrip.md',self.root/'.sys/plans/editor-core/orphan.md')
        self.expect('unindexed plan')

    def test_path_traversal(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['create'].append('crates/editor-core/../../escape.rs'))
        self.expect('write outside product ownership')

    def test_product_boundary_cannot_expand_authority(self):
        self.edit('.sys/black-hole/role-map.md', lambda d: d['editor-core']['product'].append('spec/'))
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['create'].append('spec/unauthorized.md'))
        self.expect('not an exclusive product-owned boundary')

    def test_internal_symlink_cannot_escape_role(self):
        directory = self.root/'crates/editor-core'
        directory.mkdir(parents=True, exist_ok=True)
        (directory/'alias').symlink_to(self.root/'spec', target_is_directory=True)
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['create'].append('crates/editor-core/alias/unauthorized.md'))
        self.expect('write traverses symlink')

    def test_directory_inventory_rejected(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['create'].append('crates/editor-core/nested/'))
        self.expect('inventory must name an exact file')

    def test_false_completed_rejected(self):
        self.edit('.sys/backlogs/editor-core.md', lambda d: d[0].update(state='completed',claim='none',attempt=0,result='none'))
        self.expect('completed without attempted verification evidence')

    def add_dependency(self, state='blocked', owner='quality'):
        source = self.root/'.sys/plans/editor-core/editor-core-001-document-roundtrip.md'
        target = self.root/'.sys/plans/quality/quality-001-reference.md'
        target.write_text(source.read_text())
        self.edit(target.relative_to(self.root), lambda d: d.update(id='QUALITY-001',role='quality',create=['tests/reference/probe.py'],modify=[],dependencies=[]))
        self.edit('.sys/backlogs/quality.md', lambda d: d.append({'id':'QUALITY-001','plan':str(target.relative_to(self.root)),'state':state,'claim':'none','attempt':1,'result':'Reference launch unavailable; rerun the isolated GUI probe when an authorized launch path exists.'}))
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['dependencies'].append({'role':owner,'plan':str(target.relative_to(self.root)),'unblock':'python3 tests/reference/probe.py'}))
        return target

    def test_ready_with_unresolved_dependency_rejected(self):
        self.add_dependency()
        self.edit('.sys/backlogs/editor-core.md', lambda d: d[0].update(state='ready',claim='none'))
        self.expect('executable work has unresolved dependency')

    def test_realistic_blocked_dependency_and_durable_recovery(self):
        self.add_dependency()
        self.edit('.sys/backlogs/editor-core.md', lambda d: d[0].update(state='blocked',claim='none',result='QUALITY-001 owns missing reference evidence. Resume after that plan completes and python3 tests/reference/probe.py succeeds.'))
        self.assertEqual([],validate(self.root))
        self.edit('.sys/backlogs/quality.md',lambda d:d[0].update(state='completed',result='Synthetic fixture: probe passed; this test does not claim real reference observation.'))
        self.edit('.sys/backlogs/editor-core.md',lambda d:d[0].update(state='ready',result='Synthetic resolution evidence recorded and prerequisite command checked.'))
        self.assertEqual([],validate(self.root))

    def test_dependency_owner_mismatch_rejected(self):
        self.add_dependency(state='completed',owner='desktop')
        self.expect('dependency owner mismatch')

    def test_cancelled_dependency_is_not_satisfied(self):
        self.add_dependency(state='cancelled')
        self.edit('.sys/backlogs/editor-core.md',lambda d:d[0].update(state='ready',claim='none'))
        self.expect('executable work has unresolved dependency')

    def test_cycle_rejected_even_when_both_blocked(self):
        target=self.add_dependency()
        self.edit('.sys/backlogs/editor-core.md',lambda d:d[0].update(state='blocked',claim='none',result='Waiting on QUALITY-001'))
        self.edit(target.relative_to(self.root),lambda d:d['dependencies'].append({'role':'editor-core','plan':'.sys/plans/editor-core/editor-core-001-document-roundtrip.md','unblock':'cargo test'}))
        self.expect('dependency cycle')


if __name__ == '__main__':
    unittest.main()
