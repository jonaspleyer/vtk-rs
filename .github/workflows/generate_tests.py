oss = ["ubuntu-24.04", "ubuntu-22.04", "macos-13", "macos-14"]
toolchains = ["stable", "beta", "nightly"]


def os_to_features_packages(os):
    if os == "ubuntu-24.04":
        features = "--features vtk9-1"
        packages = "libvtk9.1 libvtk9-dev"
    elif os == "ubuntu-22.04":
        features = "--features vtk9-1"
        packages = "libvtk9.1 libvtk9-dev"
    elif os == "macos-13":
        features = "--features vtk9-4"
        packages = "vtk"
    elif os == "macos-14":
        features = "--features vtk9-4"
        packages = "vtk"
    else:
        raise ValueError("Unknown Operating System")
    return features, packages


for toolchain in toolchains:
    for os in oss:
        os_re = os.replace(".", "_")
        filename = f"test_{toolchain}_{os_re}.yml"
        features, packages = os_to_features_packages(os)
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
      features: {features}
"""
        with open(filename, "w") as f:
            f.write(contents)

print("| | " + " | ".join(toolchains) + " | Build |")
print("|---" * (len(toolchains) + 1) + "|---|")
for os in oss:
    os_re = os.replace(".", "_")
    line = f"| `{os}` | "
    features, _ = os_to_features_packages(os)
    for toolchain in toolchains:
        badge_md = f"![{toolchain}-{os_re}](https://img.shields.io/github/\
actions/workflow/status/jonaspleyer/vtk-rs/\
test_{toolchain}_{os_re}.yml?style=flat-square&label=CI)"
        line += badge_md + " |"
    line += f"`cargo build {features}` |"
    print(line)
