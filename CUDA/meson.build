project('cuda', 'c',
  version : '0.1',
  default_options : ['warning_level=3'])

project_headers = [
    'test.h',
]

project_source_files = [
    'test.c',
]

project_target = shared_library(
    meson.project_name(),
    project_source_files,
    install : true,
)

pkg_mod = import('pkgconfig')
pkg_mod.generate(
    name : meson.project_name(),
    filebase : meson.project_name(),
    description : 'test',
    subdirs : meson.project_name(),
    libraries : project_target,
)