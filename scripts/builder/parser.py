import argparse

def parse_args():
    argparser = argparse.ArgumentParser(description = "Build Esque using this tool")

    mode_group = argparser.add_mutually_exclusive_group()

    mode_group.add_argument("--release", default = False, action= "store_true", help="Builds the Project in Release Mode")
    mode_group.add_argument("--debug", default = False, action= "store_true", help="Builds the Project in Debug Mode")

    argparser.add_argument("--documentation", default = True, action = "store_true", help="Recompiles the documentation of all projects")
    argparser.add_argument('--features',
                        type=lambda x: x.split(':'),
                        default=[],
                        help='Which features the kernel should be built with. Format: --features feat1:feat2:feat3')

    argparser.add_argument('--modules',
                        type=lambda x: x.split(':'),
                        default=[],
                        help='Which modules the kernel should be built with. Format: --modules feat1:feat2:feat3')

    argparser.add_argument("--no-modules", default = False, action= "store_true", help="Selects whether the project should be built without any modules")

    argparser.add_argument("--custom-initramfs", default=None)

    argparser.add_argument("--arch", default="",help="Which architecture the kernel should be built for")
    return argparser.parse_args()