"""Compare two complete local p0_corpus runs by PDF content, independently of filenames."""
import argparse
from collections import Counter
import json
from pathlib import Path
import statistics


def load_run(directory):
    summary = json.loads((directory / "summary.json").read_text(encoding="utf8"))
    records = json.loads((directory / "report.json").read_text(encoding="utf8"))
    by_digest = {r["sha256"]: r for r in records}
    if not records or len(by_digest) != len(records) or summary["tested"] != len(records):
        raise ValueError("Incomplete report or duplicate inputs")
    return summary, by_digest


def outcome(record):
    if not record["source_unchanged"] or record.get("unchanged") is False or record.get("undo_unchanged") is False:
        return "integrity_failure"
    if record["status"] != "ok":
        return record["status"]
    if "differential" in record:
        return record["differential"]["status"]
    return record.get("archive_xml", "ok")


def comparison(before, after):
    a, left = load_run(before)
    b, right = load_run(after)
    if left.keys() != right.keys() or a["phase"] != b["phase"]:
        raise ValueError("Different inputs or operations")
    transitions = Counter()
    deltas = []
    for key, original in left.items():
        revised = right[key]
        transitions[(outcome(original), outcome(revised))] += 1
        deltas.append(revised["wall_seconds"] - original["wall_seconds"])
    return {
        "tested": len(right),
        "phase": b["phase"],
        "transitions": [
            {"before": old, "after": new, "count": count}
            for (old, new), count in sorted(transitions.items())
        ],
        "before_median_seconds": a["median_seconds"], "before_p95_seconds": a["p95_seconds"],
        "after_median_seconds": b["median_seconds"], "after_p95_seconds": b["p95_seconds"],
        "median_paired_delta_seconds": round(statistics.median(deltas), 3),
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("before", type=Path)
    parser.add_argument("after", type=Path)
    options = parser.parse_args()
    print(json.dumps(comparison(options.before, options.after), indent=2))
