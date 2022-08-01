import argparse
import sys

import yaml

from .demo import demo
from .emlyml import Model as SvgEventModel


def main():
    parser = argparse.ArgumentParser(prog="emlyml")
    subparsers = parser.add_subparsers(help="commands", dest="command")
    compile_parser = subparsers.add_parser(
        "compile", help="compile a yaml model into svg"
    )
    compile_parser.add_argument(
        "infile",
        help="yaml input file",
        nargs="?",
        type=argparse.FileType("r"),
        default=sys.stdin,
    )
    compile_parser.add_argument(
        "outfile",
        help="svg output file",
        nargs="?",
        type=argparse.FileType("w"),
        default=sys.stdout,
    )
    demo_parser = subparsers.add_parser("demo", help="generate a demo model.yaml")
    demo_parser.add_argument(
        "outfile",
        help="yaml output file",
        nargs="?",
        type=argparse.FileType("w"),
        default=sys.stdout,
    )
    args = parser.parse_args()
    match args.command:
        case "compile":
            parsed_yml = yaml.safe_load(args.infile)
            model = SvgEventModel.from_yaml(parsed_yml)
            args.outfile.write(str(model.render()))
        case "demo":
            args.outfile.write(demo())
        case _:
            raise NotImplementedError()
