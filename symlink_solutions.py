# %%
from os import PathLike
from pathlib import Path


def link_soln(src_dir: PathLike[str], filename: PathLike[str], dst_dir: PathLike[str]):
    src_dir = Path(src_dir)
    dst_dir = Path(dst_dir)

    soln_file = (src_dir / filename).resolve()
    link_dst = dst_dir / f"{src_dir.name}_{filename}"
    link_dst.symlink_to(soln_file)


def main():
    src_path = Path("src")
    for (filename, dst_dir) in [
        ("soln.adoc", "pages"),
        ("mod.rs", "examples"),
        ("input.txt", "attachments"),
    ]:
        dst_path = Path(f"solutions/modules/ROOT/{dst_dir}")
        dst_path.mkdir(parents=True, exist_ok=True)

        for f in dst_path.iterdir():
            if f.is_symlink():
                f.unlink()

        for day in range(1, 26):
            folder_name = f"day_{day:02d}"
            link_soln(src_path / folder_name, filename, dst_path)

    # Path(f"solutions/modules/ROOT/examples/main.rs")


if __name__ == "__main__":
    main()
