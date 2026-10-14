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
        shutil.copytree(source, self.root, ignore=shutil.ignore_patterns('.git', 'target', '__pycache__'))

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

    def test_real_scaffold_and_checker_is_read_only(self):
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

    def test_readonly_write_conflict(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['read_only'].append(d['create'][0]))
        self.expect('read-only/write conflict')

    def test_unindexed_plan(self):
        shutil.copy(self.root/'.sys/plans/editor-core/editor-core-001-document-roundtrip.md',self.root/'.sys/plans/editor-core/orphan.md')
        self.expect('unindexed plan')

    def test_path_traversal(self):
        self.edit('.sys/plans/editor-core/editor-core-001-document-roundtrip.md', lambda d: d['create'].append('crates/editor-core/../../escape.rs'))
        self.expect('write outside product ownership')


if __name__ == '__main__':
    unittest.main()
