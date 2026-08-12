from .main import main
from .interpreter import repl
import sys


def cli():
    if len(sys.argv) > 1 and sys.argv[1] == "repl":
        repl()
    else:
        main()


if __name__ == '__main__':
    cli()
