#!/usr/bin/env python3


def main():
    print("AXIOM project bootstrap")
    print("Modules:")
    for m in [
        "kernel",
        "runtime",
        "compilers",
        "pkg",
        "tools",
        "infra",
        "docs",
    ]:
        print(f"- {m}")

    print("\nTo run proto VM: python -m runtime.proto_vm.main")


if __name__ == "__main__":
    main()
