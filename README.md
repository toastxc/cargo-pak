## Cargo-pak
the easiest way to package flatpak files


## How-to

### Dependancies
You will need 
- Rust stable (as well as cargo and other tools)
- cargo-pak
- flatpak-builder
- mold
```bash
rustup update
cargo install cargo-pak
apt-get install flatpak-builder mold
```

### Create a rust application
```bash
cargo new hello
cd hello
```
```rust
fn main() {
    println!("hello world!");
}
```



### Create an app config
**The file must be named `pak.toml`!!**


This config contains details for both the flatpak manifest and .Desktop file. the following example is for a graphical `X11` based application.
```toml
app_id="xyz.toastxc.Hello"
app_name= "hello"
# defined in Cargo.toml (release is performant)
profile="release"

# cargo-pack will default to package name in Cargo.toml
# bin="hello-world"

# definitions: https://docs.flatpak.org/en/latest/sandbox-permissions.html
permissions = [
    "--share=network",
    "--socket=x11",
    "--device=dri"
]

# definitions: https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
[desktopfile]
terminal= false
```
If you want to use a CLI, it's a bit more simple. Information such as the app name and version are derived from Cargo.toml
```toml
app_id="xyz.toastxc.Hello"

[desktopfile]
terminal= true
```

### Icons
For the .desktop file icon to work you MUST leave a `.png` in the root of the directory, identical to the bin name. (e.g. `hello-world.png`)
For example
```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"
```
```bash
hello.png
```
Even if the file is NOT a png, rename it. This program can convert file types and sizes.

### Commands
These commands act in a similar way to docker-compose; they are directory dependant. You must be at the root of your rust project file for this to work

#### Generate
This command generates a desktopfile and flatpak manifest file based on pak.toml
```bash
cargo-pak generate
```
#### Build
Builds a flatpak application based on the desktop & manifest file
```bash
cargo-pak build
```
#### Install
Installs (as root) the flatpak
```bash
cargo-pak install
```
#### Remove
Removes the flatpak from your system
```bash
cargo-pak remove
```


# F.A.Q
## Why is my icon not working?
cargo-pak automatically converts and resizes icons, it should work as long as the icon follows these conditions
- square (exactly 1:1 ratio)
- in root directory (./)
Most file types are supported as well as sizes

## Building isn't working!
If you have modified files you may need to run `cargo-pak generate` again.
