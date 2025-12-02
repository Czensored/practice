from pathlib import Path

root = Path(".")
incomplete_markers = {".incomplete"}

incomplete_projects = []

for project_dir in root.rglob("*"):
    if project_dir.is_dir():
        if any((project_dir / marker).exists() for marker in incomplete_markers):
            incomplete_projects.append(project_dir.relative_to(root))

tree: dict[str, dict] = {}
incomplete_set = {p.as_posix() for p in incomplete_projects}

for path in incomplete_projects:
    parts = path.parts
    node = tree
    for part in parts:
        node = node.setdefault(part, {})

def render_tree(node: dict, prefix_parts=(), indent: int = 0, lines=None):
    if lines is None:
        lines = []
    for name in sorted(node.keys()):
        new_prefix = prefix_parts + (name,)
        full_path = "/".join(new_prefix)

        bullet_prefix = "  " * indent + "- "

        # Make the bullet a link if this directory itself is marked incomplete
        if full_path in incomplete_set:
            line = f"{bullet_prefix}[{name}](./{full_path})"
        else:
            # Just a grouping label, not itself incomplete
            line = f"{bullet_prefix}{name}"

        lines.append(line)
        render_tree(node[name], new_prefix, indent + 1, lines)

    return lines

status_lines = ["# Unfinished Projects", ""]
status_lines += render_tree(tree)

(root / "status.md").write_text("\n".join(status_lines) + "\n", encoding="utf-8")
print("status.md updated.")
