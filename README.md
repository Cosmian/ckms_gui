# Graphical user interface for the Cosmian KMS CLI

This tool downloads the main file of the KMS CLI from a precise version of the KMS, and installs a hook for Klask.

The version is defined by an env var `VERSION` or by the latest release of the KMS (fallback)

## Build

Install dependencies

```console
$ sudo apt install librust-atk-sys-dev librust-gdk-sys-dev 
```

## Run

Run the GUI

```console
$ cargo run
```
