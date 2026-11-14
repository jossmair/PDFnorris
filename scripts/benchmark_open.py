"""Benchmark synthetic PDF opening without third-party Python dependencies.

Build corpus_probe in release mode, then provide --probe and --output.
The measurement includes process startup and first-page text analysis, not
first visible paint, scrolling, memory use, or Office rendering.
"""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import platform
import statistics
import subprocess
import time


def fixture(page_count):
    objects = [b"<< /Type /Catalog /Pages 2 0 R >>", b""]
    pages = []
    for index in range(page_count):
        page = len(objects) + 1
        pages.append(f"{page} 0 R")
        text = f"BT /F1 12 Tf 40 780 Td (Synthetic reference page {index + 1}) Tj ET".encode()
        objects.extend([
            (f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] "
             f"/Resources << /Font << /F1 << /Type /Font /Subtype /Type1 "
             f"/BaseFont /Helvetica >> >> >> /Contents {page + 1} 0 R >>").encode(),
            b"<< /Length " + str(len(text)).encode() + b" >>\nstream\n" + text + b"\nendstream",
        ])
    objects[1] = f"<< /Type /Pages /Kids [{' '.join(pages)}] /Count {page_count} >>".encode()
    data = bytearray(b"%PDF-1.4\n")
    offsets = []
    for index, obj in enumerate(objects, 1):
        offsets.append(len(data))
        data.extend(f"{index} 0 obj\n".encode() + obj + b"\nendobj\n")
    xref = len(data)
    data.extend(f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode())
    for offset in offsets:
        data.extend(f"{offset:010d} 00000 n \n".encode())
    data.extend((f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R >>\n"
                 f"startxref\n{xref}\n%%EOF").encode())
    return bytes(data)


def measure(probe, directory, page_count):
    source = directory / f"synthetic-{page_count}.pdf"
    source.write_bytes(fixture(page_count))
    digest = hashlib.sha256(source.read_bytes()).digest()
    timings = []
    for _ in range(3):
        start = time.perf_counter()
        process = subprocess.run(
            [str(probe), "inspect", str(source), str(directory / "probe-output")],
            capture_output=True, timeout=60, check=True,
        )
        result = json.loads(process.stdout.decode("utf-8").splitlines()[-1])
        if result.get("status") != "ok":
            raise RuntimeError("The opening probe refused the synthetic fixture")
        timings.append((time.perf_counter() - start) * 1000)
    if hashlib.sha256(source.read_bytes()).digest() != digest:
        raise RuntimeError("The probe changed the source fixture")
    return {
        "pages": page_count,
        "bytes": source.stat().st_size,
        "open_and_first_page_analysis_ms": timings,
        "median_ms": statistics.median(timings),
        "source_unchanged": True,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--probe", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    probe = args.probe.resolve(strict=True)
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("run-%Y%m%d-%H%M%S-%f")
    directory = args.output.resolve() / stamp
    directory.mkdir(parents=True, exist_ok=False)
    results = []
    for pages in [1, 10, 100, 300, 500]:
        record = measure(probe, directory, pages)
        results.append(record)
        print(json.dumps(record), flush=True)
    report = {
        "probe_sha256": hashlib.sha256(probe.read_bytes()).hexdigest(),
        "platform": platform.platform(),
        "measurement": "process startup, open and first-page analysis; not UI rendering or RSS",
        "measurements": results,
    }
    (directory / "baseline-performance.json").write_text(json.dumps(report, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
