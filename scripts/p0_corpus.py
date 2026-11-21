"""Private-safe, sequential regression runner; source files are opened read-only.

Oracle dependencies are development tools only: pypdfium2, pypdf, numpy.
Reports contain ordinal IDs, counts and geometry, never paths or extracted text.
Windows child processes inherit a 1536 MiB process-memory Job limit. Each probe
and each independent comparison has a timeout. A skipped case is never passed.
"""
import argparse
import ctypes
import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time
import zipfile
import xml.etree.ElementTree as ET


def memory_job():
    if os.name != "nt":
        return None
    from ctypes import wintypes as w
    class Basic(ctypes.Structure):
        _fields_ = [("user", ctypes.c_int64), ("job", ctypes.c_int64),
                    ("flags", w.DWORD), ("minimum", ctypes.c_size_t),
                    ("maximum", ctypes.c_size_t), ("processes", w.DWORD),
                    ("affinity", ctypes.c_size_t), ("priority", w.DWORD), ("scheduling", w.DWORD)]
    class Extended(ctypes.Structure):
        _fields_ = [("basic", Basic), ("io", ctypes.c_uint64 * 6),
                    ("process_memory", ctypes.c_size_t), ("job_memory", ctypes.c_size_t),
                    ("peak_process", ctypes.c_size_t), ("peak_job", ctypes.c_size_t)]
    api = ctypes.WinDLL("kernel32", use_last_error=True)
    api.CreateJobObjectW.restype = w.HANDLE
    api.GetCurrentProcess.restype = w.HANDLE
    api.SetInformationJobObject.argtypes = [w.HANDLE, ctypes.c_int, ctypes.c_void_p, w.DWORD]
    api.AssignProcessToJobObject.argtypes = [w.HANDLE, w.HANDLE]
    handle = api.CreateJobObjectW(None, None)
    limit = Extended()
    limit.basic.flags = 0x100  # JOB_OBJECT_LIMIT_PROCESS_MEMORY
    limit.process_memory = 1536 * 1024 * 1024
    if not handle or not api.SetInformationJobObject(handle, 9, ctypes.byref(limit), ctypes.sizeof(limit)) or not api.AssignProcessToJobObject(handle, api.GetCurrentProcess()):
        raise RuntimeError("Cannot enforce the Windows process memory budget")
    return handle  # Keep it for this process and its children.


def compare(source, result, record):
    import numpy as np
    import pypdfium2 as pdfium
    from pypdf import PdfReader
    a, b = PdfReader(source, strict=True), PdfReader(result, strict=True)
    assert len(a.pages) == len(b.pages), "page_count"
    # Text edits must leave page dictionaries (except content/resources) and
    # document navigation/interactive structures semantically unchanged.
    def plain(value, seen=None):
        seen = set() if seen is None else seen
        if hasattr(value, "get_object") and hasattr(value, "idnum"):
            key = (value.idnum, value.generation)
            resolved = value.get_object()
            if isinstance(resolved, dict) and resolved.get("/Type") == "/Page":
                return ["page_reference", *key]
            if key in seen:
                return "cycle"
            return plain(resolved, seen | {key})
        if isinstance(value, dict):
            return {str(k): plain(v, seen) for k, v in value.items() if str(k) not in {"/Parent", "/P", "/Length"}}
        if isinstance(value, list):
            return [plain(x, seen) for x in value]
        return str(value)
    for left, right in zip(a.pages, b.pages):
        for key in ["/MediaBox", "/CropBox", "/Rotate", "/UserUnit", "/Annots"]:
            assert plain(left.get(key)) == plain(right.get(key)), "page_structure"
    for key in ["/Outlines", "/Names", "/AcroForm", "/Metadata"]:
        assert plain(a.trailer["/Root"].get(key)) == plain(b.trailer["/Root"].get(key)), "catalogue_structure"
    pixels = changed = geometric = inside = 0
    with pdfium.PdfDocument(source) as old, pdfium.PdfDocument(result) as new:
        for index in range(len(old)):
            p, q = old[index], new[index]
            assert p.get_size() == q.get_size(), "page_size"
            scale = min(1.5, 1400 / max(p.get_size()))
            bitmap1, bitmap2 = p.render(scale=scale), q.render(scale=scale)
            image1, image2 = bitmap1.to_numpy(), bitmap2.to_numpy()
            diff = np.max(np.abs(image1.astype(np.int16) - image2.astype(np.int16)), axis=2)
            keep = np.ones(diff.shape, dtype=bool)
            if index == record["page"]:
                x0, y0, x1, y1 = record["mask_view"]
                # Two PDF points of explicit antialias/ink padding, no global SSIM.
                left, top = max(0, int((x0-2)*scale)), max(0, int((y0-2)*scale))
                right, bottom = max(0, int((x1+2)*scale+1)), max(0, int((y1+2)*scale+1))
                keep[top:bottom, left:right] = False
                inside += int(np.count_nonzero((diff > 2) & ~keep))
            changed += int(np.count_nonzero((diff > 2) & keep))
            pixels += int(np.count_nonzero(keep))
            # Independent glyph coordinates outside the edited region.
            def glyphs(page, bitmap):
                text = page.get_textpage()
                transform = bitmap.get_posconv(page)
                found = []
                for i in range(text.count_chars()):
                    bounds = text.get_charbox(i)
                    x, y = transform.to_bitmap((bounds[0]+bounds[2])/2, (bounds[1]+bounds[3])/2)
                    if index != record["page"] or not (left <= x < right and top <= y < bottom):
                        found.append((text.get_text_range(i, 1), tuple(round(v, 2) for v in bounds)))
                return found
            geometric += glyphs(p, bitmap1) != glyphs(q, bitmap2)
    return {"status": "unexpected_difference" if changed or geometric else ("ok" if inside else "no_visible_edit"),
            "changed_pixels_inside": inside,
            "unchanged_region_pixels": pixels, "changed_pixels_over_2": changed,
            "pages_with_glyph_displacement": geometric, "structure": "ok"}


def run(command, timeout):
    try:
        p = subprocess.run(command, capture_output=True, timeout=timeout,
                           creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
        if p.returncode:
            return {"status": "process_failed", "exit_code": p.returncode}
        return json.loads(p.stdout.decode("utf8").splitlines()[-1])
    except subprocess.TimeoutExpired:
        return {"status": "timeout"}
    except OSError as exc:
        return {"status": "launch_error", "code": exc.errno}
    except (ValueError, IndexError):
        return {"status": "invalid_probe_result"}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--sources", type=Path, nargs="+")
    ap.add_argument("--probe", type=Path)
    ap.add_argument("--oracle-dir", type=Path, required=True)
    ap.add_argument("--output", type=Path)
    ap.add_argument("--limit", type=int)
    ap.add_argument("--cohort", type=Path, help="Use exactly the input hashes from a previous report.json")
    ap.add_argument("--expect-fixtures", type=Path, help="Check every move result against the synthetic manifest")
    ap.add_argument("--timeout", type=int, default=90)
    ap.add_argument("--phase", choices=["move", "delete", "docx", "odt", "ocr"], default="move")
    ap.add_argument("--keep", action="store_true")
    ap.add_argument("--compare", nargs=3)
    args = ap.parse_args()
    sys.path.insert(0, str(args.oracle_dir.resolve()))
    job = memory_job()
    if args.compare:
        source, result, record = args.compare
        try:
            print(json.dumps(compare(source, result, json.loads(Path(record).read_text(encoding="utf8")))))
        except Exception as exc:
            print(json.dumps({"status": "oracle_error", "category": type(exc).__name__}))
        return
    if not args.sources or not args.probe or not args.output:
        ap.error("--sources, --probe and --output are required")
    output = args.output.resolve()
    if any(not s.exists() or (s.is_dir() and output.is_relative_to(s.resolve())) for s in args.sources):
        ap.error("A source is missing or contains the output directory")
    if args.limit is not None and args.limit < 1:
        ap.error("--limit must be positive")
    files = sorted({p.resolve() for s in args.sources for p in (s.rglob("*.pdf") if s.is_dir() else [s])})
    if not args.probe.is_file() or not files:
        ap.error("Probe or input files missing")
    sources = {}
    for source in files:
        with source.open("rb") as stream:
            digest = hashlib.file_digest(stream, "sha256").hexdigest()
        sources.setdefault(digest, source)
    digests = sorted(sources)[:args.limit]
    if args.cohort:
        if args.limit is not None:
            ap.error("Specify either --cohort or --limit")
        digests = [row["sha256"] for row in json.loads(args.cohort.read_text(encoding="utf8"))]
        if not digests or len(set(digests)) != len(digests) or any(key not in sources for key in digests):
            ap.error("Previous cohort is incomplete or contains duplicate entries")
    output.mkdir(parents=True, exist_ok=False)
    probe = args.probe.resolve()
    if os.name == "nt" and probe.suffix.lower() == ".exe":
        # Cargo may relink its example while a long corpus run is in progress.
        probe = Path(shutil.copyfile(probe, output / "probe.exe"))
    with probe.open("rb") as stream:
        metadata = {"probe_sha256": hashlib.file_digest(stream, "sha256").hexdigest(),
                    "phase": args.phase, "timeout_seconds": args.timeout,
                    "python": sys.version.split()[0],
                    "oracle": {name: importlib.metadata.version(name)
                               for name in ("pypdfium2", "pypdf", "numpy")}}
    (output / "environment.json").write_text(json.dumps(metadata, indent=2), encoding="utf8")
    records = []
    expected = {}
    if args.expect_fixtures:
        if args.phase != "move":
            ap.error("Synthetic manifest describes move operations")
        for row in json.loads(args.expect_fixtures.read_text(encoding="utf8")):
            with (args.expect_fixtures.parent / row["file"]).open("rb") as stream:
                expected[hashlib.file_digest(stream, "sha256").hexdigest()] = row["expected"]
        if set(expected) != set(digests):
            ap.error("Fixture inputs differ from the manifest")
    for digest in digests:
        source = sources[digest]
        ident = f"case-{len(records)+1:04d}"
        dest = output / ident
        started = time.monotonic()
        native = args.phase in {"move", "delete"}
        command = [str(probe), str(source), str(dest), args.phase] if native else [str(probe), args.phase, str(source), str(dest)]
        record = run(command, args.timeout)
        record["id"] = ident
        record["sha256"] = digest
        with source.open("rb") as stream:
            record["source_unchanged"] = hashlib.file_digest(stream, "sha256").hexdigest() == digest
        # Diagnostic probes can contain source-derived error messages: omit those.
        record.pop("error", None)
        if record.get("status") == "ok" and native:
            pending = output / "pending.json"
            pending.write_text(json.dumps(record), encoding="utf8")
            record["differential"] = run([sys.executable, str(Path(__file__).resolve()), "--oracle-dir", str(args.oracle_dir.resolve()), "--compare", str(source), str(dest/"result.pdf"), str(pending)], args.timeout)
            pending.unlink()
        elif record.get("status") == "ok" and args.phase in {"docx", "odt"}:
            try:
                with zipfile.ZipFile(dest/f"export.{args.phase}") as z:
                    assert z.testzip() is None
                    for name in z.namelist():
                        if name.endswith(".xml"):
                            ET.fromstring(z.read(name))
                record["archive_xml"] = "ok"
            except Exception:
                record["archive_xml"] = "error"
        record["wall_seconds"] = round(time.monotonic()-started, 3)
        if expected:
            record["meets_expectation"] = (
                record["source_unchanged"] and record["status"] == expected[digest]
                and record.get("unchanged", True) and record.get("undo_unchanged", True)
                and (record["status"] != "ok" or record["differential"]["status"] == "ok")
            )
        records.append(record)
        (output/"report.json").write_text(json.dumps(records, indent=2), encoding="utf8")
        if not args.keep and dest.is_dir():
            assert dest.parent == output and dest.name == ident
            shutil.rmtree(dest)  # Only this newly-created output, never a source.
        if len(records) % 10 == 0:
            print(json.dumps({"completed":len(records)}), flush=True)
    durations = sorted(r["wall_seconds"] for r in records)
    summary = {"tested":len(records), "phase":args.phase, "memory_limit_mib":1536 if job else None,
               "median_seconds":statistics.median(durations) if durations else None,
               "p95_seconds":durations[min(len(durations)-1, int(len(durations)*.95))] if durations else None,
               "statuses":{s:sum(r["status"]==s for r in records) for s in sorted({r["status"] for r in records})},
               "differential":{s:sum(r.get("differential",{}).get("status")==s for r in records) for s in sorted({r["differential"]["status"] for r in records if "differential" in r})}}
    if expected:
        summary["fixture_failures"] = sum(not r["meets_expectation"] for r in records)
    (output/"summary.json").write_text(json.dumps(summary, indent=2), encoding="utf8")
    print(json.dumps(summary))
    if summary.get("fixture_failures") or any(not r["source_unchanged"] for r in records):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
