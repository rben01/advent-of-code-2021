# %%
from os import PathLike
from pathlib import Path


def link_soln(src_dir: PathLike[str], dst_dir: PathLike[str]):
    src_dir = Path(src_dir)
    dst_dir = Path(dst_dir)

    soln_file = (src_dir / "soln.adoc").resolve()
    link_dst = dst_dir / f"{src_dir.name}.adoc"
    link_dst.symlink_to(soln_file)


def main():
    src_path = Path("src")
    dst_path = Path("solutions/modules/ROOT/pages")
    for day in range(1, 26):
        folder_name = f"day_{day:02d}"
        link_soln(src_path / folder_name, dst_path)


if __name__ == "__main__":
    main()
