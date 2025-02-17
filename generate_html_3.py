import base64
import subprocess
from pathlib import Path
import re

containing_folder = Path(__file__).parent
release = containing_folder / "target" / "wasm32-unknown-unknown" / "release"

print("Running cargo build")
subprocess.run("cargo build --release --target wasm32-unknown-unknown", check=True)
print("Running wasm-bindgen")
subprocess.run(["wasm-bindgen", release / f"{containing_folder.name}.wasm", "--out-dir", release / "wasm_soup", "--target", "web", "--no-typescript"], check=True)
print("Finished wasm-bindgen")

wasm_base64 = base64.b64encode(open(release / "wasm_soup" / f"{containing_folder.name}_bg.wasm", "rb").read()).decode()
js = open(release / "wasm_soup" / f"{containing_folder.name}.js").read().replace("\\", "\\\\").replace("`", "\\`").replace("$", "\\$")
snippets = set(re.findall(rf"'./(snippets/{containing_folder.name}-([^/]+)/inline\d+.js)'", js))

html = f"""<html lang="en">
<head>
    <meta charset="utf-8">
    <title>TITLE</title>
</head>
<body style="margin:0;padding:0;background-color:black">
    <canvas width="150" height="150", id="canvas"></canvas>
    <script type="module" data-info="https://stackoverflow.com/a/43834063">
    const wasm_blob_url = URL.createObjectURL(new Blob([Uint8Array.from(atob("{wasm_base64}"), c => c.charCodeAt(0))], {{type: "application/wasm"}}));
    console.log("WASM blob created");
    {"".join(f"""
const js_{name} = `{open(release / "wasm_soup" / path).read().replace("\\", "\\\\").replace("`", "\\`").replace("$", "\\$")}`;
const js_url_{name} = URL.createObjectURL(new Blob([js_{name}], {{type: "application/javascript"}}));
const script_{name} = document.createElement('script');
script_{name}.type = "module";
script_{name}.src = js_url_{name};
script_{name}.textContent = js_{name};
document.body.appendChild(script_{name});
""" for (path, name) in snippets)}
    const js_bg = `{js}`.replace("{containing_folder.name}_bg.wasm", wasm_blob_url){"".join(f".replace('./{path}', js_url_{name})" for (path, name) in snippets)};
    const js_blob_url = URL.createObjectURL(new Blob([js_bg], {{type: "application/javascript"}}));
    const script = document.createElement('script');
    script.type = "module";
    script.src = js_blob_url;
    script.textContent = js_bg;
    document.body.appendChild(script);
    console.log("Bindgen js added");
    const js_script = document.createElement('script');
    js_script.type = "module";
    const js = `import init from "${{js_blob_url}}";await init();`
    js_script.src = URL.createObjectURL(new Blob([js], {{type: "application/javascript"}}));
    js_script.textContent = js;
    document.body.appendChild(js_script);
    console.log("Initial script done");
    </script>

</body>

</html>"""

with open(release / "index.html", "w") as f:
    f.write(html)