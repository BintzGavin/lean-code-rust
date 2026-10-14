"""Validate catalog/fixture integrity only. This is not an editor parity test."""
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
catalog = json.loads((root / 'catalog.json').read_text())
behaviors = {row['id']: row for row in catalog['behaviors']}
scenarios = {row['id']: row for row in catalog['scenarios']}
assert len(behaviors) == len(catalog['behaviors']) == 20
assert len(scenarios) == len(catalog['scenarios']) == 64
for bid, behavior in behaviors.items():
    assert len(behavior['scenario_ids']) >= 2
    for sid in behavior['scenario_ids']:
        assert bid in scenarios[sid]['behavior_ids']
for scenario in scenarios.values():
    assert all(bid in behaviors for bid in scenario['behavior_ids'])
    assert all(scenario['id'] in behaviors[bid]['scenario_ids'] for bid in scenario['behavior_ids'])
    assert scenario['execution_status'] == 'blocked'
    assert scenario['expected_current_result'] is None
    assert scenario['actual_result'] is None
    assert scenario['oracle_status'] == 'unknown'
    assert scenario['actions'] and scenario['observation_points']
    assert scenario['setup_instructions'] and scenario['state_dependency']
    for eid in scenario['evidence_ids']:
        assert eid in (root / 'evidence.md').read_text()
    for uid in scenario['remaining_questions']:
        assert uid in (root / 'gaps.md').read_text()
manifest = json.loads((root / 'fixtures/manifest.json').read_text())
assert len(manifest) == 4
for name, expected in manifest.items():
    payload = (root / 'fixtures' / name).read_bytes()
    assert len(payload) == expected['bytes']
    assert hashlib.sha256(payload).hexdigest() == expected['sha256']
assert catalog['coverage']['directly_observed'] == 0
assert catalog['coverage']['blocked'] == len(scenarios)
assert catalog['coverage']['behavior_areas'] == len(behaviors)
assert catalog['coverage']['scenario_cells'] == len(scenarios)
assert catalog['coverage']['product_tests'] == catalog['coverage']['model_executions'] == 0

assert len(catalog["extension_fixture_families"]) == 12
for family in catalog["extension_fixture_families"]:
    assert len(family["scenario_ids"]) == 2
    assert all(scenarios[sid]["fixture"] == family["id"] for sid in family["scenario_ids"])
assert catalog["requirement_decisions"][0]["id"] == "D-0003"
print('PASS: 20 behaviors, 64 blocked/unknown scenarios, 12 proposed extension fixture families, 4 text fixture hashes. Product scenarios executed: 0.')
