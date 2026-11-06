#!/usr/bin/env python3
"""Tests for `ops/scripts/live_audit.py`.

The engine writes the three generated ledgers and answers `--check`. Both are
load-bearing: a stale ledger is a document that states a measurement the tree no
longer supports, and a `--check` that passes on a stale ledger is worse than no
check. So the tests are aimed at the two failure modes that a green run hides:
the check that compares nothing, and the renderer that loses a row.

Run with:

    python3 -m unittest discover -s ops/scripts/tests -v
"""

import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[1]
REPO = SCRIPTS.parents[1]
sys.path.insert(0, str(SCRIPTS))

import live_audit  # noqa: E402  (the import needs the path above)


def package(number, paths=(), tests=(), closes=()):
    return {
        "id": "WP-%02d" % number,
        "title": "Package %d" % number,
        "lane": "A",
        "layer": "T-Opening",
        "tlg": True,
        "depends_on": [],
        "milestone": "M0",
        "lock_paths": list(paths),
        "card_tests": list(tests),
        "closes": list(closes),
    }


def ledger():
    return {
        "schema": 1,
        "package": [
            package(1, paths=["docs/audit/live/lane-a.md"], tests=["a_test"]),
            package(2, paths=["src/missing/"], tests=["b_test"])
        ],
        "tlg": [
            {"id": "TLG-03", "requirement": "no legacy storage",
             "evidence": "no_legacy_storage", "packages": ["WP-02"]},
        ],
        "finding": [
            {"id": "BLG-001", "severity": "Critical", "family": "ledger",
             "closes_with": ["WP-01"], "locations": [], "trace": "2/2"},
            {"id": "BULGU-001", "severity": "High", "family": "archive",
             "closes_with": ["WP-02"], "locations": ["src/missing/mod.rs"]},
        ],
    }


def build_tree(root):
    """A tiny repository: two Rust files, a present lock path, no missing one."""
    (root / "src").mkdir(parents=True)
    (root / "docs/audit/live").mkdir(parents=True)
    (root / "src/lib.rs").write_text(
        "fn a_test() {\n    assert!(true);\n}\n", encoding="utf-8")
    (root / "src/other.rs").write_text(
        "pub fn helper() -> u32 {\n    1\n}\n", encoding="utf-8")
    (root / "docs/audit/live/lane-a.md").write_text("# Lane A\n", encoding="utf-8")


class TreeTests(unittest.TestCase):
    def test_brace_expansion_lists_every_alternative(self):
        self.assertEqual(
            live_audit.expand_braces("src/registry/{role,permissionless}.rs"),
            ["src/registry/role.rs", "src/registry/permissionless.rs"])

    def test_a_bare_identifier_is_not_a_path(self):
        self.assertFalse(live_audit.is_path_spec("min_stake"))
        self.assertTrue(live_audit.is_path_spec("src/validators/"))
        self.assertTrue(live_audit.is_path_spec("config/naming/concept_map.toml"))

    def test_a_package_state_follows_paths_and_tests(self):
        cases = [
            ((0, 0, 0, 0, 0), "no lock path or test on the card"),
            ((2, 2, 0, 0, 0), "record or document package"),
            ((2, 2, 2, 2, 0), "delivered, evidence pending"),
            ((2, 1, 2, 2, 0), "partial"),
            ((2, 2, 2, 1, 0), "partial"),
            ((2, 0, 2, 0, 0), "not started"),
        ]
        for arguments, expected in cases:
            self.assertEqual(live_audit.package_state(*arguments), expected, arguments)

    def test_a_present_lock_path_counts_and_a_missing_one_does_not(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            tree = live_audit.Tree(str(root))
            self.assertTrue(tree.present("docs/audit/live/lane-a.md"))
            self.assertFalse(tree.present("src/missing/"))
            self.assertTrue(tree.present("src/*.rs"))

    def test_a_test_name_is_found_through_the_function_index(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            tree = live_audit.Tree(str(root))
            self.assertTrue(tree.defines_fn("a_test"))
            self.assertFalse(tree.defines_fn("a_test_that_does_not_exist"))
            self.assertEqual(tree.defining_files("a_test"), ["src/lib.rs"])


class MeasurementTests(unittest.TestCase):
    def test_a_missing_lock_path_makes_the_package_partial(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            tree = live_audit.Tree(str(root))
            first = live_audit.measure_package(tree, ledger()["package"][0])
            second = live_audit.measure_package(tree, ledger()["package"][1])
            self.assertEqual(first["state"], "delivered, evidence pending")
            self.assertEqual(first["tests_present"], 1)
            self.assertEqual(first["tests_owned"], 0)
            self.assertEqual(second["state"], "not started")
            self.assertEqual(second["paths_present"], 0)

    def test_a_finding_without_a_reproduction_test_is_unverified(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            tree = live_audit.Tree(str(root))
            packages = {"WP-01": live_audit.measure_package(tree, ledger()["package"][0]),
                        "WP-02": live_audit.measure_package(tree, ledger()["package"][1])}
            first = live_audit.measure_finding(tree, ledger()["finding"][0], packages)
            second = live_audit.measure_finding(tree, ledger()["finding"][1], packages)
            self.assertEqual(first["state"], "closure candidate, reproduction test missing")
            self.assertEqual(second["state"], "unverified")
            self.assertEqual(second["location_state"], "recorded file gone")

    def test_an_unprobed_launch_item_is_not_green(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            tree = live_audit.Tree(str(root))
            item = {"id": "TLG-99", "requirement": "unknown", "evidence": "none",
                    "packages": []}
            result = live_audit.measure_tlg(tree, item, [])
            self.assertEqual(result["state"], "unverifiable")


class DocumentTests(unittest.TestCase):
    def test_a_cell_escapes_its_pipe(self):
        self.assertEqual(live_audit.cell("a | b"), "a \\| b")
        self.assertEqual(live_audit.cell("  "), "-")

    def test_a_document_round_trips_through_its_markers(self):
        text = live_audit.compose("# Title\n", "body\n")
        parts = live_audit.split_generated(text)
        self.assertIsNotNone(parts)
        self.assertEqual(parts[1].strip("\n"), "body")
        self.assertTrue(parts[0].startswith("# Title"))

    def test_write_then_check_is_stable(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            data = ledger()
            docs = live_audit.build_documents(str(root), data, None)
            written = live_audit.write_documents(str(root), docs)
            self.assertEqual(len(written), 3)
            self.assertEqual(live_audit.check_documents(str(root), docs), [])
            again = live_audit.write_documents(str(root), docs)
            self.assertEqual(again, [])

    def test_a_stale_block_is_reported(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            data = ledger()
            docs = live_audit.build_documents(str(root), data, None)
            live_audit.write_documents(str(root), docs)
            # The tree changes and the committed ledger does not follow.
            (root / "src/missing").mkdir()
            (root / "src/missing/mod.rs").write_text("pub fn x() {}\n", encoding="utf-8")
            stale = live_audit.build_documents(str(root), data, None)
            problems = live_audit.check_documents(str(root), stale)
            self.assertGreaterEqual(len(problems), 1)
            self.assertTrue(any(live_audit.PACKAGES_DOC in p for p in problems))
            self.assertTrue(all("does not match the measurement" in p for p in problems))

    def test_a_document_without_markers_is_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            docs = live_audit.build_documents(str(root), ledger(), None)
            live_audit.write_documents(str(root), docs)
            (root / live_audit.PACKAGES_DOC).write_text("# empty\n", encoding="utf-8")
            problems = live_audit.check_documents(str(root), docs)
            self.assertTrue(any("no generated block markers" in p for p in problems))

    def test_the_rendered_ledgers_obey_the_dash_and_english_rules(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            build_tree(root)
            docs = live_audit.build_documents(str(root), ledger(), None)
            live_audit.write_documents(str(root), docs)
            banned = "\u2010\u2011\u2012\u2013\u2014\u2015\u2212\u00ad\u2e3a\u2e3b\ufe58\ufe63\uff0d"
            turkish = "şğıçöüŞĞİÇÖÜ"
            for relative in (live_audit.PACKAGES_DOC, live_audit.LIVE_DOC,
                             live_audit.TLG_DOC):
                text = (root / relative).read_text(encoding="utf-8")
                for character in banned:
                    self.assertNotIn(character, text, relative)
                for character in turkish:
                    self.assertNotIn(character, text, relative)


class SnapshotTests(unittest.TestCase):
    def test_the_snapshot_carries_the_directive_numbering(self):
        path = REPO / live_audit.LEDGER_PATH
        if not path.is_file():
            self.skipTest("no snapshot in this checkout")
        data = json.loads(path.read_text(encoding="utf-8"))
        self.assertEqual(live_audit.structural_checks(str(REPO), data), [])
        self.assertEqual(len(data["package"]), 64)
        self.assertEqual(len(data["tlg"]), 27)
        self.assertEqual(len(data["finding"]), 311)

    def test_the_committed_ledgers_are_current(self):
        if not (REPO / live_audit.LEDGER_PATH).is_file():
            self.skipTest("no snapshot in this checkout")
        self.assertEqual(live_audit.main(["--root", str(REPO), "--check"]), 0)


if __name__ == "__main__":
    unittest.main()
