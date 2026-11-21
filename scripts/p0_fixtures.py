"""Reproducible A–Z synthetic PDFs, original test content under the project licence.

Uses development-only reportlab/pypdf. Embedded font: the existing attributed
Inter face. No system font, private document or third-party sample is copied.
The manifest distinguishes refusal/non-applicability from editable cases.
"""
import argparse
import io
import json
from pathlib import Path
import sys


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("output", type=Path)
    ap.add_argument("--oracle-dir", type=Path, required=True)
    a = ap.parse_args()
    sys.path.insert(0, str(a.oracle_dir))
    from reportlab.pdfgen import canvas
    from reportlab.pdfbase import pdfmetrics
    from reportlab.pdfbase.ttfonts import TTFont
    from reportlab.lib.utils import ImageReader
    from PIL import Image
    from pypdf import PdfReader, PdfWriter
    from pypdf.generic import DictionaryObject as D, NameObject as N, ArrayObject as A, NumberObject as I, TextStringObject as T, DecodedStreamObject, EncodedStreamObject
    root = Path(__file__).resolve().parents[1]
    pdfmetrics.registerFont(TTFont("InterFixture", str(root/"vendor/pdfcraft/assets/fonts/Inter-Regular.ttf")))
    a.output.mkdir(parents=True, exist_ok=False)
    descriptions = [
        "Standard font and real-valued page origin", "Fragmented text operators", "Text split across streams",
        "Individually positioned characters", "Offset fake bold", "Accents and ligatures",
        "Indices superscripts and fractions", "Embedded subset font", "Mixed fonts",
        "Rotated text and transform", "Text in Form XObject", "Nested Form XObjects",
        "Bezier paths", "Text clipping mask", "Transparency and stacking",
        "Table and borders", "Geometry diagram", "Mathematical formula",
        "Two columns", "Mixed page sizes", "Link annotation and bookmark",
        "AcroForm widget", "Signature marker (not cryptographically signed)",
        "Raster scan (no native text)", "Damaged Flate content", "Oversized expanded stream",
    ]
    manifest = []
    for index, description in enumerate(descriptions):
        letter = chr(ord("A")+index)
        buf = io.BytesIO()
        c = canvas.Canvas(buf, pagesize=(320,240), invariant=1)
        c.setFont("Helvetica", 14)
        if letter not in "BCDEJKLNX":
            c.drawString(20, 190, "TARGET")
        if letter == "B":
            t = c.beginText(20,190); t.textOut("TAR"); t.textOut("GET"); c.drawText(t)
        if letter == "C":
            c.drawString(20,190,"TARGET")
        if letter == "D":
            for n, ch in enumerate("TARGET"):
                c.drawString(20+n*12,190,ch)
        if letter == "E":
            c.drawString(20,190,"TARGET"); c.drawString(20.5,190,"TARGET")
        if letter in "FGHIR":
            c.setFont("InterFixture",14)
            c.drawString(20,120,"Été cœur ﬁancée −12 / 7")
        if letter in "GR":
            t=c.beginText(20,85); t.textOut("x"); t.setRise(6); t.setFont("InterFixture",9); t.textOut("2"); t.setRise(0); t.textOut(" + H"); t.setRise(-3); t.textOut("2"); t.setRise(0); t.textOut("O"); c.drawText(t)
            c.drawString(180,115,"12"); c.line(175,108,210,108); c.drawString(185,89,"7")
        if letter == "I":
            c.setFont("Courier",12); c.drawString(180,60,"SECOND FONT")
        if letter == "J":
            c.saveState(); c.translate(60,150); c.rotate(20); c.drawString(0,0,"TARGET"); c.restoreState()
        if letter in "KL":
            c.beginForm("Inner",0,0,200,100); c.setFont("Helvetica",14); c.drawString(20,50,"TARGET"); c.endForm()
            if letter == "L":
                c.beginForm("Outer",0,0,200,100); c.doForm("Inner"); c.endForm()
            c.doForm("Inner" if letter=="K" else "Outer")
        if letter == "M":
            for n in range(20):
                p=c.beginPath(); p.moveTo(20,20+n); p.curveTo(100,140,200,-20,300,30+n); c.drawPath(p)
        if letter == "N":
            c.saveState(); t=c.beginText(20,170); t.setFont("Helvetica",28); t.setTextRenderMode(7); t.textOut("TARGET"); c.drawText(t); c.setFillColorRGB(0,0,1); c.rect(0,0,320,240,fill=1,stroke=0); c.restoreState()
        if letter == "O":
            c.saveState(); c.setFillAlpha(.3); c.setFillColorRGB(1,0,0); c.rect(10,100,160,45,fill=1); c.setFillColorRGB(0,0,1); c.rect(60,80,160,45,fill=1); c.restoreState()
        if letter == "P":
            c.grid([20,100,180,260],[30,60,90,120]); c.drawString(30,100,"CELL")
        if letter == "Q":
            p=c.beginPath(); p.moveTo(20,30); p.lineTo(170,30); p.lineTo(80,130); p.close(); c.drawPath(p); c.drawString(20,15,"A"); c.drawString(170,15,"B")
        if letter == "S":
            c.drawString(20,130,"LEFT"); c.drawString(190,130,"RIGHT")
        if letter == "T":
            c.showPage(); c.setPageSize((240,320)); c.drawString(20,200,"PORTRAIT")
        if letter == "U":
            c.bookmarkPage("first"); c.addOutlineEntry("Synthetic bookmark","first"); c.linkURL("https://example.invalid/fixture",(20,30,120,50)); c.textAnnotation("Synthetic annotation",(180,30,210,60))
        if letter == "V":
            c.acroForm.textfield(name="synthetic",value="VALUE",x=20,y=60,width=150,height=24)
        if letter == "X":
            c.drawImage(ImageReader(Image.new("RGB",(128,96),(70,120,180))),0,0,320,240)
        if letter not in "KLX":
            c.setFont("Helvetica", 8)
            c.drawString(270, 10, letter)
        c.save()
        reader=PdfReader(buf); writer=PdfWriter(); writer.clone_document_from_reader(reader)
        page=writer.pages[0]
        expected="ok"
        if letter == "A":
            from pypdf.generic import FloatObject
            page.mediabox[0] = FloatObject(0)
        if letter == "C":
            data=page.get_contents().get_data(); cut=data.index(b"(TARGET)")
            refs=[]
            for part in [data[:cut],data[cut:]]:
                stream=DecodedStreamObject(); stream.set_data(part); refs.append(writer._add_object(stream))
            page[N("/Contents")]=A(refs)
        if letter == "W":
            sig=D({N("/Type"):N("/Sig"),N("/ByteRange"):A([I(0),I(1),I(2),I(3)])})
            writer._root_object[N("/SyntheticSignature")]=writer._add_object(sig)
            expected="refused_edit"
        if letter in "YZ":
            old=page.raw_get("/Contents")
            if letter=="Y":
                stream=EncodedStreamObject(); stream[N("/Filter")]=N("/FlateDecode"); stream._data=b"broken-flate"
            else:
                stream=DecodedStreamObject(); stream.set_data(b" "*(16*1024*1024+1)); stream=stream.flate_encode()
            page[N("/Contents")]=A([old,writer._add_object(stream)])
            expected="refused_inventory"
        if letter == "N": expected="refused_edit"
        if letter in "KLX": expected="not_applicable"  # No direct native-text edit invented.
        path=a.output/f"{letter}.pdf"
        with path.open("wb") as target: writer.write(target)
        manifest.append({"file":path.name,"description":description,"operation":"move_first_native_block_3_2","expected":expected,"invariants":["unchanged pixels outside target", "glyph geometry outside target", "page geometry", "annotations and catalogue", "exact undo redo", "reopen"]})
    (a.output/"manifest.json").write_text(json.dumps(manifest,indent=2),encoding="utf8")
    print(json.dumps({"generated":len(manifest)}))


if __name__ == "__main__": main()
