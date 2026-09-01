"""Package manager for Project Kolos."""

import json
from pathlib import Path


class PackageManager:
    def __init__(self, workspace_dir="."):
        self.workspace_dir = Path(workspace_dir)

    def init_package(
        self,
        name: str,
        version: str = "0.1.0",
        author: str = "",
    ):
        manifest = {
            "name": name,
            "version": version,
            "author": author,
            "main": "main.kolos",
            "dependencies": {},
        }
        manifest_path = self.workspace_dir / "kolos.json"
        with open(manifest_path, "w", encoding="utf-8") as f:
            json.dump(manifest, f, indent=2)

        main_file = self.workspace_dir / "main.kolos"
        if not main_file.exists():
            with open(main_file, "w", encoding="utf-8") as f:
                f.write(f'// Package {name}\nprint "Hello from {name}!";\n')

        print(f"Initialized Kolos package '{name}' at {manifest_path}")

    def load_manifest(self):
        manifest_path = self.workspace_dir / "kolos.json"
        if not manifest_path.exists():
            raise FileNotFoundError("kolos.json manifest not found.")
        with open(manifest_path, "r", encoding="utf-8") as f:
            return json.load(f)

    def list_dependencies(self):
        manifest = self.load_manifest()
        deps = manifest.get("dependencies", {})
        print(f"Package: {manifest.get('name')} v{manifest.get('version')}")
        print(f"Dependencies ({len(deps)}):")
        for pkg, ver in deps.items():
            print(f"  - {pkg}: {ver}")
        return deps
