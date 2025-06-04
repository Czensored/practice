from pathlib import Path

# Define the root path and marker filenames
root = Path(".")
incomplete_markers = {".incomplete", "TODO.md", "WIP"}

# Find directories containing any marker files
incomplete_projects = []

for project_dir in root.rglob("*"):
    if project_dir.is_dir():
        if any((project_dir / marker).exists() for marker in incomplete_markers):
            incomplete_projects.append(project_dir.relative_to(root))

# Write the status.md file
status_lines = ["# Unfinished Projects\n"]
status_lines += [f"- {str(path)}" for path in sorted(incomplete_projects)]

(root / "status.md").write_text("\n".join(status_lines) + "\n")
print("status.md updated.")

