# Assemble (powered by 🦀)
![CI](https://github.com/Nate1729/assemble/actions/workflows/main.yml/badge.svg)

This is a small tool for teachers to create student groups. This
tool allows for students to be excluded from being in the same group.

This means that if Alice and Bob should work together, `assemble`
will make sure they aren't placed in the same group.

# Warning
This is currently in beta so use at your own risk.

# Cross Compilation
This is a work-around until I have automated releases working.
`cargo build --target x86_64-pc-windows-gnu` builds a window's
static binary.
