import importlib.util

def __bootstrap__():
    import sys, pkg_resources
    file_path = pkg_resources.resource_filename(__name__, 'bin/rust.so')
    spec = importlib.util.spec_from_file_location(__name__, file_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

__bootstrap__()