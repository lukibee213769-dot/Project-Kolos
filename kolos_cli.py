"""Kolos CLI entrypoint for developer convenience."""
import argparse
import sys


def run_bootstrap():
    import bootstrap
    bootstrap.main()


def run_repl():
    from runtime.proto_vm.interpreter import repl
    repl()


def run_sample():
    from runtime.proto_vm.main import run_file
    print(run_file('tests/sample.asm'))


def main(argv=None):
    if argv is None:
        argv = sys.argv[1:]
    p = argparse.ArgumentParser(prog='kolos')
    sub = p.add_subparsers(dest='cmd')
    sub.add_parser('bootstrap')
    sub.add_parser('repl')
    sub.add_parser('run-sample')
    sub.add_parser('tests', help='run unit tests')
    args = p.parse_args(argv)
    if args.cmd == 'bootstrap':
        run_bootstrap()
    elif args.cmd == 'repl':
        run_repl()
    elif args.cmd == 'run-sample':
        run_sample()
    elif args.cmd == 'tests':
        import subprocess
        cmd = [
            sys.executable,
            '-m',
            'unittest',
            'discover',
            '-s',
            'tests',
            '-p',
            'test_*.py',
            '-v',
        ]
        subprocess.run(cmd)
    else:
        p.print_help()


if __name__ == '__main__':
    main()
