from pathlib import Path
import base64

folder = Path(__file__).parent

wasm_b64 = base64.b64encode(open(folder / "target" / "wasm32-unknown-unknown" / "debug", f"{folder.name}.wasm", "rb").read())

with open(folder / "wasm_soup" / "bideo_bame.js") as js_file:
    js_file = js_file.read()
    js_file.replace("fetch(module_or_path)", 'new Response(Uint8Array.from(atob("' + str(wasm_b64)[2:-1] + '"), c => c.charCodeAt(0)).buffer,{headers: new Headers({"content-type":"application/wasm"})})')
    