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


def run_script(path: str):
    if path.endswith('.asm'):
        from runtime.proto_vm.main import run_file
        print(run_file(path))
    else:
        from runtime.proto_vm.pipeline import execute_file
        res = execute_file(path)
        if res is not None:
            print("Program returned:", res)


def compile_script(path: str):
    from runtime.proto_vm.pipeline import compile_source
    with open(path, 'r', encoding='utf-8') as f:
        source = f.read()
    instructions = compile_source(source)
    print(f"Compiled {len(instructions)} instructions:")
    for i, (op, arg) in enumerate(instructions):
        if arg is not None:
            print(f"  {i:04d}: {op:<10} {arg}")
        else:
            print(f"  {i:04d}: {op}")


def main(argv=None):
    if argv is None:
        argv = sys.argv[1:]
    p = argparse.ArgumentParser(prog='kolos')
    sub = p.add_subparsers(dest='cmd')
    sub.add_parser('bootstrap')
    sub.add_parser('repl')
    sub.add_parser('run-sample')
    sub.add_parser('tests', help='run unit tests')

    p_run = sub.add_parser('run', help='run a .kolos or .asm source file')
    p_run.add_argument('file', help='path to file')

    p_compile = sub.add_parser('compile', help='compile a .kolos source file to bytecode')
    p_compile.add_argument('file', help='path to file')

    p_fmt = sub.add_parser('fmt', help='format a .kolos source file')
    p_fmt.add_argument('file', help='path to file')

    p_lint = sub.add_parser('lint', help='lint a .kolos source file')
    p_lint.add_argument('file', help='path to file')

    p_pkg = sub.add_parser('pkg', help='manage packages')
    p_pkg_sub = p_pkg.add_subparsers(dest='pkg_cmd')
    p_pkg_init = p_pkg_sub.add_parser('init', help='initialize a new package')
    p_pkg_init.add_argument('name', help='package name')

    args = p.parse_args(argv)
    if args.cmd == 'bootstrap':
        run_bootstrap()
    elif args.cmd == 'repl':
        run_repl()
    elif args.cmd == 'run-sample':
        run_sample()
    elif args.cmd == 'run':
        run_script(args.file)
    elif args.cmd == 'compile':
        compile_script(args.file)
    elif args.cmd == 'fmt':
        from tools.formatter import format_file
        format_file(args.file)
    elif args.cmd == 'lint':
        from tools.linter import lint_file
        lint_file(args.file)
    elif args.cmd == 'pkg':
        from pkg.pm import PackageManager
        pm = PackageManager()
        if args.pkg_cmd == 'init':
            pm.init_package(args.name)
        else:
            p_pkg.print_help()
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
