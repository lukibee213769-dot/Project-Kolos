def main():
    print("Proto VM: Hello from runtime proto VM")


def run_file(path: str):
    from ..bytecode.loader import run_file as run_bc_file
    return run_bc_file(path)


if __name__ == "__main__":
    main()
