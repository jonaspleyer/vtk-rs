oss = ["ubuntu-24.04", "ubuntu-22.04", "ubuntu-20.04", "macos-13", "macos-14"]
toolchains = ["stable", "beta", "nightly"]


def os_to_features(os):
    features = ""
    if os == "ubuntu-24.04":
        features = "--features vtk9-1"
    elif os == "ubuntu-22.04":
        features = "--features vtk9-1"
    elif os == "ubuntu-20.04":
        features = "--features vtk9-1"
    elif os == "macos-13":
        features = "--features vtk9-4"
    elif os == "macos-14":
        features = "--features vtk9-4"
    return features


for toolchain in toolchains:
    for os in oss:
        filename = "test_{}_{}.yml".format(toolchain, os)
        features = os_to_features(os)
        contents = f"""\
on: [push, pull_request]

name: Test-Suite {toolchain} {os}

jobs:
  CI-{toolchain}-{os}:
    uses: ./.github/workflows/reuse.yml
    with:
      toolchain: {toolchain}
      os: {os}
      features: {features}
"""
        with open(filename, "w") as f:
            f.write(contents)

print("| | " + " | ".join(toolchains) + " | Build |")
print("|---" * (len(toolchains) + 1) + "|---|")
for os in oss:
    line = f"| `{os}` | "
    for toolchain in toolchains:
        badge_md = f"![{toolchain}-{os}](https://img.shields.io/github/\
actions/workflow/status/jonaspleyer/vtk-rs/\
test_{toolchain}_{os}.yml?style=flat-square&label=CI)"
        line += badge_md + " |"
    line += f"`cargo build {os_to_features(os)}` |"
    print(line)
