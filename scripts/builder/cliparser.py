import argparse
import subcommands as sc

SUBCOMMANDS = [
    "build",
    "clean",
    "clippy",
    "all",
    "format",
    "run",
    "build-kernel",
    "build-boot",
    "strip",
    "image",
    "initramfs",
    "cloc",
    "count-unsafe",
    "doc",
    "setup",
    "test"
]

SUBCOMMANDS_TO_FN = {
    "build": sc.build,
    "clean": sc.clean,
    "clippy": sc.clippy,
    "all": sc.all,
    "format": sc.format,
    "run": sc.run_qemu,
    "build-kernel": sc.build_kernel,
    "build-boot": sc.build_boot,
    "strip": sc.strip,
    "image": sc.image,
    "initramfs": sc.initramfs,
    "cloc": sc.cloc,
    "count-unsafe": sc.count_unsafe,
    "doc": sc.build_docs,
    "setup": sc.setup,
    "test": sc.test,
}

def parse_args():
    argparser = argparse.ArgumentParser(description = "Build Esque using this tool")

    mode_group = argparser.add_mutually_exclusive_group()

    mode_group.add_argument("--release", default = False, action= "store_true", help="Builds the Project in Release Mode")
    mode_group.add_argument("--debug", default = False, action= "store_true", help="Builds the Project in Debug Mode")

    argparser.add_argument("--documentation", default = False, action = "store_true", help="Recompiles the documentation of all projects")
    argparser.add_argument('--kernel-features',
                        type=lambda x: x.split(':'),
                        default=[],
                        help='Which features the kernel should be built with. Format: --features feat1:feat2:feat3')

    argparser.add_argument('--boot-features',
                        type=lambda x: x.split(':'),
                        default=[],
                        help='Which features the bootloader should be built with. Format: --features feat1:feat2:feat3')


    argparser.add_argument('--modules',
                        type=lambda x: x.split(':'),
                        default=[],
                        help='Which modules the kernel should be built with. Format: --modules feat1:feat2:feat3')

    argparser.add_argument("--no-modules", default = False, action= "store_true", help="Selects whether the project should be built without any modules")

    argparser.add_argument("--custom-initramfs", default=None)

    argparser.add_argument("subcommand", choices=SUBCOMMANDS)

    argparser.add_argument("--arch", default="",help="Which architecture the kernel should be built for")
    argparser.add_argument("--config", default="Esque.toml", help="Selects the config to use")
    
    run_group = argparser.add_mutually_exclusive_group()
    run_group.add_argument("--run", default=False,help="Should the project be run after building?", action="store_true")
    run_group.add_argument("--never-run", default=False, help="Ignore any running, even when ./y.py run is invoked", action="store_true")

    argparser.add_argument("--disable-never-run", default=False, help="Should the 'never-run' parameter in the config be ignored?", action="store_true")
    argparser.add_argument("--outimage", default="config", help="Specifies the name and location of the produced image")
    argparser.add_argument("--minimal-toolchain", default=False, help="Use this option if you are building using the minimal toolchain", action="store_true")

    return argparser.parse_args()