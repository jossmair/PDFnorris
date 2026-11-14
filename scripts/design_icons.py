"""Original 24-unit precision pictograms. Regenerate SVGs without raster assets.

Shared optical grid, 1.35-unit outlines, charcoal enamel and copper highlights.
Geometry is intentionally authored here, independently of the previous family.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "assets/icons"
ART = {
    "open": '<path class="surface" d="M3 8V5.5A1.5 1.5 0 0 1 4.5 4H9l2 2h8a2 2 0 0 1 2 2v1"/><path class="metal" d="M2 10h20l-2.5 9H4z"/><path class="glint" d="M5 12h13"/>',
    "save": '<path class="surface" d="M4 3h13l3 3v15H4z"/><path class="metal" d="M7 3h9v6H7z"/><path d="M8 21v-8h9v8M10 16h5M10 18h5"/><path class="glint" d="M13 4.5v3"/>',
    "print": '<path d="M6 8V3h12v5"/><rect class="surface" x="2.5" y="8" width="19" height="9" rx="2"/><path class="metal" d="M6 13h12v8H6z"/><path class="glint" d="M8.5 16h7M8.5 18.5h5"/><circle class="dot" cx="18.5" cy="11" r=".8"/>',
    "select": '<path class="metal" d="m5 2 14 11-7 .8-3.2 6.7z"/><path class="glint" d="m7 5 7 5.5"/><path d="m13 15 3 6"/>',
    "hand": '<path class="surface" d="M7 12V6a1.25 1.25 0 0 1 2.5 0v5-7a1.25 1.25 0 0 1 2.5 0v7-6a1.25 1.25 0 0 1 2.5 0v6-4a1.25 1.25 0 0 1 2.5 0v7c0 4.5-2.5 7-6 7-2.5 0-4-1.5-5-3L3 13c-1-2 1-3 2-1l2 2"/><path class="detail" d="M10 14v3m3-3v3m3-3v2"/>',
    "edit": '<path class="surface" d="m4 15 10-10 5 5L9 20l-6 1z"/><path class="metal" d="m14 5 2-2a2 2 0 0 1 3 0l2 2a2 2 0 0 1 0 3l-2 2z"/><path class="detail" d="m6 15 3 3m-2-4 8-8"/><path d="M3 21h6"/>',
    "text": '<path d="M4 6V3h16v3M12 3v18M8 21h8"/><path class="detail" d="M6 5h4m4 0h4"/>',
    "image": '<rect class="surface" x="2.5" y="3.5" width="19" height="17" rx="2"/><circle class="metal" cx="8" cy="9" r="2"/><path class="metal" d="m4 18 5-5 3 3 4-7 4 9z"/><path class="glint" d="M5 6h3M18 6h1"/>',
    "drawing": '<path class="surface" d="m5 15 9-10 5 5-10 9-6 2z"/><path class="metal" d="m14 5 2-2 5 5-2 2z"/><path class="detail" d="m5 16 3 3M4 5h5M6.5 2.5v5"/>',
    "line": '<path d="m5 19 14-14"/><rect class="surface" x="2.5" y="16.5" width="5" height="5" rx="1"/><rect class="metal" x="16.5" y="2.5" width="5" height="5" rx="1"/>',
    "circle": '<circle class="surface" cx="12" cy="12" r="8.5"/><path class="glint" d="M6.5 9A6 6 0 0 1 10 6"/>',
    "square": '<rect class="surface" x="4" y="4" width="16" height="16" rx="1.5"/><path class="glint" d="M7 7h5"/>',
    "rectangle": '<rect class="surface" x="2.5" y="6" width="19" height="12" rx="1.5"/><path class="glint" d="M5.5 9h5"/>',
    "table": '<rect class="surface" x="2.5" y="3.5" width="19" height="17" rx="1.5"/><path class="metal" d="M3 4h18v5H3z"/><path d="M3 14.5h18M9 9v11M15.5 9v11"/><path class="glint" d="M5 6.5h3m3 0h2m3 0h3"/>',
    "highlight": '<path class="surface" d="m5 13 9-10 6 6-9 10z"/><path class="metal" d="m5 13 6 6-3 2H3v-5z"/><path class="detail" d="m8 13 7-7"/><path d="M3 23h17"/>',
    "comment": '<path class="surface" d="M4 3.5h16a1.5 1.5 0 0 1 1.5 1.5v11a1.5 1.5 0 0 1-1.5 1.5h-9L5 21v-3.5H4A1.5 1.5 0 0 1 2.5 16V5A1.5 1.5 0 0 1 4 3.5z"/><path d="M6 8h12M6 12h8"/><path class="glint" d="M6 6h5"/>',
    "search": '<circle class="surface" cx="10" cy="10" r="6.5"/><path d="m15 15 6 6"/><path class="glint" d="M6.5 9A4 4 0 0 1 9 6.5"/><path class="detail" d="m17 16 4 4"/>',
    "copy": '<path class="surface" d="M8 3h10l3 3v13H8z"/><path class="metal" d="M3 7h10l3 3v12H3z"/><path class="glint" d="M6 12h7M6 15h5M6 18h6"/>',
    "paste": '<path class="surface" d="M6 5H4v17h16V5h-2"/><rect class="metal" x="8" y="2" width="8" height="5" rx="1.5"/><path d="M8 12h8M8 16h6"/><path class="glint" d="M10 4h4"/>',
    "delete": '<path d="M3 6h18M9 6V3h6v3"/><path class="surface" d="m5 6 1 15h12l1-15"/><path class="detail" d="m9 10 .5 7m5.5-7-.5 7"/>',
    "page": '<path class="surface" d="M5 2.5h9L20 8v13.5H5z"/><path class="metal" d="M14 2.5V8h6"/><path d="M8 12h9M8 15h9M8 18h6"/>',
    "pages": '<path d="M3 17V3h11"/><path class="surface" d="M7 6h9l5 5v11H7z"/><path class="metal" d="M16 6v5h5"/><path d="M10 15h8M10 18h6"/>',
    "blank-page": '<path class="surface" d="M4 2.5h9L19 8v8"/><path d="M4 2.5v19h7M13 3v5h6"/><circle class="metal" cx="17" cy="18" r="5"/><path class="glint" d="M17 15.5v5M14.5 18h5"/>',
    "add-pdf": '<path d="M3 16V3h10"/><path class="surface" d="M6 5h8l5 5v4M6 5v15h5M14 5v5h5"/><circle class="metal" cx="17" cy="18" r="5"/><path class="glint" d="M17 15.5v5M14.5 18h5"/>',
    "office-export": '<path class="surface" d="M3 2.5h9l5 5v6M3 2.5v19h9M12 3v5h5"/><path d="M6 11h6M6 14h4M6 17h3"/><path d="M13 18h9m-3-3 3 3-3 3"/>',
    "export": '<path class="surface" d="M4 12v9h16v-9"/><path d="M12 16V3m-5 5 5-5 5 5"/><path class="detail" d="M7 18v1h10v-1"/>',
    "crop": '<path d="M3 7h14v14M7 3v14h14"/><path class="detail" d="m3 21 18-18"/>',
    "grid": '<rect class="surface" x="3" y="3" width="18" height="18" rx="1.5"/><path d="M3 9h18M3 15h18M9 3v18M15 3v18"/>',
    "options": '<path class="detail" d="M3 6h18M3 12h18M3 18h18"/><rect class="metal" x="6" y="3.5" width="4" height="5" rx="1"/><rect class="metal" x="14" y="9.5" width="4" height="5" rx="1"/><rect class="metal" x="8" y="15.5" width="4" height="5" rx="1"/>',
    "info": '<circle class="surface" cx="12" cy="12" r="9"/><path d="M12 11v6M10 17h4"/><circle class="dot" cx="12" cy="7" r=".8"/>',
    "replace": '<path class="surface" d="M8 3h10l3 3v10H8z"/><path d="M3 8v13h12M5 13h9m-3-3 3 3-3 3"/>',
    "undo": '<path d="m8 4-5 5 5 5M3 9h10a7 7 0 0 1 0 14"/><path class="detail" d="M13 20a4 4 0 0 0 4-4"/>',
    "redo": '<path d="m16 4 5 5-5 5M21 9H11a7 7 0 0 0 0 14"/><path class="detail" d="M11 20a4 4 0 0 1-4-4"/>',
    "rotateleft": '<path d="m7 3-4 4 4 4M3 7h10a7 7 0 1 1-7 10"/><rect class="surface" x="10" y="12" width="5" height="5" rx=".8"/>',
    "rotateright": '<path d="m17 3 4 4-4 4M21 7H11a7 7 0 1 0 7 10"/><rect class="surface" x="9" y="12" width="5" height="5" rx=".8"/>',
    "move": '<path d="M12 2v20M2 12h20m-13-7 3-3 3 3m-6 14 3 3 3-3M5 9l-3 3 3 3m14-6 3 3-3 3"/><circle class="metal" cx="12" cy="12" r="2"/>',
    "width": '<path d="M3 4v16M21 4v16M6 12h12m-9-3-3 3 3 3m6-6 3 3-3 3"/><path class="detail" d="M6 5h12M6 19h12"/>',
    "apply": '<circle class="surface" cx="12" cy="12" r="9"/><path d="m7 12 3.5 3.5 7-8"/><path class="glint" d="M6 8a7 7 0 0 1 4-3"/>',
    "bold": '<path d="M7 3h6a4.5 4.5 0 0 1 0 9H7zm0 9h7a4.5 4.5 0 0 1 0 9H7z"/><path class="detail" d="M9 5v5m0 4v5"/>',
    "italic": '<path d="M10 3h9M5 21h9M15 3 9 21"/>',
    "underline": '<path d="M6 3v9a6 6 0 0 0 12 0V3M4 22h16"/><path class="detail" d="M8 4v7"/>',
    "strike": '<path d="M18 6c-1-3-10-5-11 0-1 4 10 5 11 8 2 6-9 9-12 4M3 12h18"/>',
    "superscript": '<path d="m3 8 10 13M13 8 3 21M17 4c0-3 5-3 5 0 0 2-5 4-5 6h5"/>',
    "subscript": '<path d="m3 3 10 13M13 3 3 16M17 15c0-3 5-3 5 0 0 2-5 4-5 6h5"/>',
    "bullets": '<path d="M8 6h13M8 12h10M8 18h13"/><circle class="dot" cx="3" cy="6" r="1"/><circle class="dot" cx="3" cy="12" r="1"/><circle class="dot" cx="3" cy="18" r="1"/>',
}
# Navigation uses the same grid and stroke; silhouettes stay deliberately spare.
for name,d in {
    "previous":"m15 5-7 7 7 7", "next":"m9 5 7 7-7 7",
    "up":"m5 15 7-7 7 7", "down":"m5 9 7 7 7-7",
    "plus":"M12 4v16M4 12h16", "minus":"M4 12h16",
    "minimize":"M5 17h14", "maximize":"M5 4h14v16H5z",
    "close":"m5 5 14 14M19 5 5 19",
}.items(): ART[name] = f'<path d="{d}"/>'
for name,ends in {"alignleft":[20,15,20,15],"aligncenter":[20,16,20,16],
                  "alignright":[20,15,20,15],"alignjustify":[20]*4}.items():
    pieces=[]
    for y,end in zip([4,9,14,19],ends):
        start=4
        if name=="aligncenter":start=(24-(end-4))/2;end=24-start
        if name=="alignright":start=24-end;end=20
        pieces.append(f'<path d="M{start} {y}h{end-start}"/>')
    ART[name]=''.join(pieces)

DEFS = '''<defs><linearGradient id="copper" x1="0" y1="0" x2="0" y2="1"><stop stop-color="#ffd0a1"/><stop offset=".42" stop-color="#ff922e"/><stop offset="1" stop-color="#dc5b13"/></linearGradient><linearGradient id="enamel" x1="0" y1="0" x2=".7" y2="1"><stop stop-color="#505b68"/><stop offset="1" stop-color="#202a36"/></linearGradient></defs>'''
STYLE = '''<style>.surface{fill:url(#enamel)}.metal{fill:url(#copper);stroke:#ffb473}.detail{stroke:#ffb473;stroke-width:.7;opacity:.85}.glint{stroke:#fff0dd;stroke-width:.65;opacity:.85}.dot{fill:#ffb473;stroke:none}</style>'''
for name,art in ART.items():
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24">{DEFS}{STYLE}<g fill="none" stroke="#e36c17" stroke-width="1.35" stroke-linecap="round" stroke-linejoin="round">{art}</g></svg>\n'
    (ROOT/f'{name}.svg').write_text(svg,encoding='utf-8')
missing = {p.stem for p in ROOT.glob('*.svg')} - ART.keys()
if missing: raise SystemExit(f'Unauthored icons: {sorted(missing)}')
print(f'{len(ART)} original icons regenerated.')
