This is a small cross-platform CLI I wrote to control the Legion 5 Pro (2021) keyboard settings on Linux. Originally the source code was released on the Legion discord channel (now LaptopWiki).

# Compilation
libudev is a needed dependency for linux.\
**Arch**: part of the systemd package.\
**Ubuntu**: libudev-dev package.\
\
Build with cargo:\
`cargo build --release`\
Build with nix:\
`nix build .`\

# Compatibility
Works on Linux and Windows. With help from the community, this has been tested to work on:
-   Legion 5 (Pro) 2020, 2021, 2022, 2023, 2024
-   Ideapad Gaming 3 2021, 2022, 2023, 2024

#  See also
A project using this as a base is [**Legion RGB Control**](https://github.com/4JX/L5P-Keyboard-RGB). This project offers a polished user-friendly GUI, uses stock effects as primitives to create new effects and even allows you to define your own.
