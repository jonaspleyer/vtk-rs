oss = ["ubuntu-24.04", "ubuntu-22.04", "macos-13", "macos-14"]
toolchains = ["stable", "beta", "nightly"]


def os_to_envs_packages(os):
    if os == "ubuntu-24.04":
        environment_vars = ""
        packages = "libvtk9.1 libvtk9-dev"
    elif os == "ubuntu-22.04":
        environment_vars = ""
        packages = "libvtk9.1 libvtk9-dev"
    elif os == "macos-13":
        environment_vars = "VTK_VERSION=-9.4"
        packages = "vtk"
    elif os == "macos-14":
        environment_vars = "VTK_VERSION=-9.4"
        packages = "vtk"
    else:
        raise ValueError("Unknown Operating System")
    return environment_vars, packages


for toolchain in toolchains:
    for os in oss:
        os_re = os.replace(".", "_")
        filename = f"test_{toolchain}_{os_re}.yml"
        envs, packages = os_to_envs_packages(os)
        contents = f"""\
on: [push, pull_request]

name: Test-Suite {toolchain} {os}

jobs:
  CI-{toolchain}-{os_re}:
    uses: ./.github/workflows/reuse.yml
    with:
      toolchain: {toolchain}
      os: {os}
      packages: {packages}
      environment_vars: {envs}
"""
        with open(filename, "w") as f:
            f.write(contents)

print("| | " + " | ".join(toolchains) + " | Build |")
print("|---" * (len(toolchains) + 1) + "|---|")
for os in oss:
    os_re = os.replace(".", "_")
    line = f"| `{os}` | "
    envs, _ = os_to_envs_packages(os)
    for toolchain in toolchains:
        badge_md = f"[![{toolchain}-{os_re}](https://img.shields.io/github/\
actions/workflow/status/jonaspleyer/vtk-rs/\
test_{toolchain}_{os_re}.yml?style=flat-square&label=CI)](\
https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_{toolchain}_{os_re}.yml)"
        line += badge_md + " |"
    if len(envs) == 0:
        line += "`cargo build` |"
    else:
        line += f"`{envs} cargo build` |"
    print(line)
