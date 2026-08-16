#!/usr/bin/env python3
"""Gera o manifest.json para publicar uma nova base no GitHub.
Uso:  python gerar-manifest.py caminho/da/base.json OWNER/REPO
Saída: manifest.json ao lado da base, pronto para subir no mesmo Release."""
import hashlib, json, sys, pathlib
base = pathlib.Path(sys.argv[1]); repo = sys.argv[2] if len(sys.argv) > 2 else "OWNER/revisor-ep"
txt = base.read_text(encoding="utf-8"); meta = json.loads(txt)["meta"]
man = {"base_versao": meta["versao"], "schema_version": meta["schema_version"],
       "sha256": hashlib.sha256(txt.encode()).hexdigest(),
       "url": f"https://github.com/{repo}/releases/latest/download/base.json",
       "publicado_em": meta.get("publicado_em", "")}
out = base.parent / "manifest.json"
out.write_text(json.dumps(man, indent=1, ensure_ascii=False), encoding="utf-8")
print("manifest.json gerado:"); print(json.dumps(man, indent=1, ensure_ascii=False))
