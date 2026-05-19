# Toml2CV

Unreasonably straightforward Rust tool built because manually tweaking my
`resume.typ` depending on the job application was too arduous.

This program reads a TOML file w/ pre-set fields, and fills them into a Typst
resume template, eventually outputting it, formatting it (w/ `typstyle`), and
compiling it (w/ `typst`).

The template is fixed, but allows flexibility in terms of choosing to not
include certain components within your TOML config.

## Setup

```bash
git clone https://github.com/masroof-maindak/toml2cv.git && cd toml2cv
cargo install --path .

# Or:

cargo install toml2cv
```

## Usage

```bash
# Assuming `2cv.toml` exists in the PWD; produces `output.typ`
toml2cv

# Run in another directory, and format + build Typst output file
toml2cv -d ~/Desktop/Resume/ --format --compile

# See `toml2cv -h` for more help
```

## Configuration

```toml
# 2cv.toml

name = "John Doe"
phone = "+1 234 5678910"
email = "john.doe@gmail.com"
site = "example.com"
github = "username-only"

skills = {
    langs = "C++, C, Go, Rust, x86 Assembly, MIPS Assembly, Python, Node, TypeScript, Bash, Lua, Awk",
    operating_systems = "Linux, Windows, Mac OS",
    dev_tools = "Neovim, Git, VS Code, Typst, GNU Coreutils, Caddy, Nginx, Docker, Tmux, Fish, GitHub Actions, Ansible",
    libraries = "OpenCV, NumPy, Express, Django",
}

[[institutes]]
institute = "X University of Y"
location = "Kuala Lampur, Malaysia"
degree = "Bachelors of Science"
major = "Computer Science"
desc = [
    "_SAT_: 1600 (800 R&W, 800 M)",
    "_CGPA_: 3.98 (Semester \\#4)"
]
start_date = { year = 2022, month = 8, day = 1 }
end_date = { year = 2026, month = 6, day = 15 }

[[workplaces]]
title = "Junior Backend Systems Engineer"
company = "Startup 02"
start_date = { year = 2026, month = 3, day = 1 }
end_date = { year = 2026, month = 5, day = 1 }
desc = [
    "Developed the pipeline for the Firefly BM1684x AI box, comprising a custom Nvidia Triton C++ backend, containerization, CI/CD workflows for model conversions, quantization, and exports to a ClearML instance"
]

[[workplaces]]
title = "Software Engineer"
company = "Stealth Startup"
start_date = { year = 2025, month = 1, day = 1 }
end_date = { year = 2025, month = 8, day = 1 }
desc = [
    "Developed a general-purpose analytics engine in Golang using Protobufs, supporting user event capture, A/B experimentation, feature flags, and a frontend for admin and business analytics.",
    "Other stuff I did at company X"
]

[[projects]]
title = "Qalam"
url = "https://github.com/masroof-maindak/qalam"
stack = "Rust, Static Site Generators, CI/CD"
desc = [
    "Built a static-site generator in Rust for my personal #link(\"https://masroof-maindak.github.io/\")[website], featuring a markdown-blog to html-converter, syntax highlighting, automatic releases, and compile-time file embeds."
]

[[projects]]
title = "Kinara Daryaft"
url = "https://github.com/masroof-maindak/kinara-daryaft"
stack = "C++23, STL, OpenCV, CMake, Computer Vision"
desc = [
    "Built a Canny Edge Detector from scratch, including Non-Maximum Suppression & Hysteresis Thresholding.",
    "Leveraged modern features like `span` and `expected` types, and STL algorithms, for robust and efficient code."
]

[[small_projects]]
title = "Tile2wall"
url = "https://github.com/masroof-maindak/kinara-daryaft"
desc = [
    "Created a Python CLI that tiles an input image into a wallpaper-sized output using OpenCV and NumPy."
]

[[small_projects]]
title = "Bit Scribe[rs]"
url = "https://github.com/masroof-maindak/bit-scribers"
desc = [
    "Built a library in Rust for encoding/decoding variable-length integers from raw file bytes."
]

[[hobby_projects]]
title = "Kyria Rev3"
url = "https://github.com/masroof-maindak/zmk"
desc = [
    "Built a split mechanical keyboard involving soldering, 3D printing, assembly, and firmware flashing.",
    "Configured its Zephyr-based firmware (ZMK) using Devicetree files."
]
```

## Acknowledgements

- [Typst: Simple Technical Resume](https://typst.app/universe/package/simple-technical-resume/)
