from pathlib import Path

root = Path(".")
incomplete_markers = {".incomplete"}

incomplete_projects = []

for project_dir in root.rglob("*"):
    if project_dir.is_dir():
        if any((project_dir / marker).exists() for marker in incomplete_markers):
            incomplete_projects.append(project_dir.relative_to(root))

status_lines = ["# Unfinished Projects\n"]
status_lines += [f"- [{str(path)}](./{str(path)})" for path in sorted(incomplete_projects)]

(root / "status.md").write_text("\n".join(status_lines) + "\n")
print("status.md updated.")

