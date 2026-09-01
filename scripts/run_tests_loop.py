import subprocess
import sys


def run_once():
    cmd = [
        sys.executable,
        '-m',
        'pytest',
        '-q',
    ]
    p = subprocess.run(cmd)
    return p.returncode


def main():
    failures = 0

    for i in range(20):
        print(f"Run {i + 1}/20")

        rc = run_once()

        if rc != 0:
            print(f"Test run {i + 1} failed with code {rc}")
            failures += 1
        else:
            print(f"Run {i + 1}/20 passed")

    if failures:
        print(f"{failures} runs failed")
        sys.exit(1)

    print("All 20 runs passed")


if __name__ == '__main__':
    main()
