#!/usr/bin/env python3
"""Derive apps/sdkwork-missory-h5 from the PC tree (run from repo root)."""
import os
import shutil

SRC = "apps/sdkwork-missory-pc"
DST = "apps/sdkwork-missory-h5"
SKIP_PACKAGES = {
    "sdkwork-missory-pc-people",
    "sdkwork-missory-pc-memories",
    "sdkwork-missory-pc-assistant",
}
SKIP_DIRS = {"node_modules", "dist", ".dart_tool", "target", "build"}

REPLACEMENTS = [
    ("@sdkwork/missory-pc-core", "@sdkwork/missory-h5-core"),
    ("@sdkwork/missory-pc-commons", "@sdkwork/missory-h5-commons"),
    ("@sdkwork/missory-pc-shell", "@sdkwork/missory-h5-shell"),
    ("@sdkwork/missory-pc", "@sdkwork/missory-h5"),
    ("sdkwork-missory-pc-core", "sdkwork-missory-h5-core"),
    ("sdkwork-missory-pc-commons", "sdkwork-missory-h5-commons"),
    ("sdkwork-missory-pc-shell", "sdkwork-missory-h5-shell"),
    ("sdkwork-missory-pc", "sdkwork-missory-h5"),
    ("MissoryPcRuntimeConfig", "MissoryH5RuntimeConfig"),
    ("MissoryPcRuntime", "MissoryH5Runtime"),
    ("loadMissoryPcRuntimeConfig", "loadMissoryH5RuntimeConfig"),
    ("parseMissoryPcRuntimeConfig", "parseMissoryH5RuntimeConfig"),
    ("createMissoryPcRuntime", "createMissoryH5Runtime"),
    ("bootstrapMissoryPcRuntime", "bootstrapMissoryH5Runtime"),
    ("platform: \"pc\"", "platform: \"h5\""),
    ("127.0.0.1:3910", "127.0.0.1:5197"),
    ("port: 3910", "port: 5197"),
    ("port: 4910", "port: 5917"),
]


def transform(text):
    for old, new in REPLACEMENTS:
        text = text.replace(old, new)
    return text


if os.path.exists(DST):
    shutil.rmtree(DST)

for base, dirs, files in os.walk(SRC):
    dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
    rel = os.path.relpath(base, SRC)
    rel_parts = rel.replace(os.sep, "/").split("/")
    if len(rel_parts) >= 2 and rel_parts[0] == "packages" and rel_parts[1] in SKIP_PACKAGES:
        dirs[:] = []
        continue
    target_dir = os.path.join(DST, rel) if rel != "." else DST
    os.makedirs(target_dir, exist_ok=True)
    for file in files:
        src_file = os.path.join(base, file)
        dst_file = os.path.join(target_dir, file)
        if file.endswith((".ts", ".tsx", ".json", ".md", ".mjs", ".html", ".css")):
            text = open(src_file, encoding="utf-8").read()
            open(dst_file, "w", encoding="utf-8", newline="\n").write(transform(text))
        else:
            shutil.copy2(src_file, dst_file)

for old_name, new_name in [
    ("sdkwork-missory-pc-commons", "sdkwork-missory-h5-commons"),
    ("sdkwork-missory-pc-core", "sdkwork-missory-h5-core"),
    ("sdkwork-missory-pc-shell", "sdkwork-missory-h5-shell"),
]:
    os.rename(
        os.path.join(DST, "packages", old_name),
        os.path.join(DST, "packages", new_name),
    )

shell_screens = os.path.join(DST, "packages", "sdkwork-missory-h5-shell", "src", "screens")
os.makedirs(shell_screens, exist_ok=True)
for cap, screen in [
    ("people", "PeopleListScreen.tsx"),
    ("people", "PersonDetailScreen.tsx"),
    ("memories", "MemoriesScreen.tsx"),
    ("assistant", "AssistantScreen.tsx"),
]:
    src = os.path.join(SRC, "packages", f"sdkwork-missory-pc-{cap}", "src", "screens", screen)
    text = open(src, encoding="utf-8").read()
    text = transform(text)
    open(os.path.join(shell_screens, screen), "w", encoding="utf-8", newline="\n").write(text)

print("H5 tree derived")
