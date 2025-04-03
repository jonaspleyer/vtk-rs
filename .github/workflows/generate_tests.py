oss = ["ubuntu-latest", "macos-13", "macos-14", "windows-latest"]
toolchains = ["stable", "beta", "nightly"]


for toolchain in toolchains:
    for os in oss:
        filename = "test_{}_{}.yml".format(toolchain, os)
        features = ""
        if os == "ubuntu-latest":
            features = "--features vtk9-1"
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

print("| | " + " | ".join(toolchains) + " |")
print("|---" * (len(toolchains) + 1) + "|")
for os in oss:
    line = f"| `{os}` | "
    for toolchain in toolchains:
        badge_md = f"![{toolchain}-{os}](https://img.shields.io/github/\
actions/workflow/status/jonaspleyer/vtk-rs/\
test_{toolchain}_{os}.yml?style=flat-square&label=CI)"
        line += badge_md + " |"
    print(line)
