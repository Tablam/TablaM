+++
title = "Installation"

+++

To install/test the last version of **TablaM**, you can opt for:

### 1. Run in the browser

Visit [https://repl.it/@mamcx/RelpIt]( https://repl.it/@mamcx/RelpIt) and remember to click in the green button "**Run**".

### 2. Install binaries

Download the [last binaries](https://github.com/Tablam/TablaM/releases/latest), select according to your operative system (available only for 64 bit): 

#### Linux

[Download the executable tablam-linux-amd64](https://github.com/Tablam/TablaM/releases/download/v0.3.1-alpha/tablam-linux-amd64), and rename it to *tablam*, then set the executable permissions. 

From the command line (bash or equivalent):

```bash
wget -O tablam https://github.com/Tablam/TablaM/releases/download/v0.3.1-alpha/tablam-linux-amd64

chmod +x tablam

./tablam
```

#### MacOS

[Download the executable tablam-macos-amd64](https://github.com/Tablam/TablaM/releases/download/v0.3.1-alpha/tablam-macos-amd64), and rename it to *tablam*, then set the executable permissions. 

From the command line (bash or equivalent):

```bash
wget -O tablam https://github.com/Tablam/TablaM/releases/download/v0.3.1-alpha/tablam-macos-amd64

chmod +x tablam

./tablam
```

#### Windows

[Download the executable tablam-windows-amd64.exe](https://github.com/Tablam/TablaM/releases/download/v0.3.1-alpha/tablam-windows-amd64.exe), and rename it to *tablam*, then click on it. 

From the command line (powershell):

```powershell
iwr "https://github.com/Tablam/TablaM/releases/download/v0.3.1-alpha/tablam-windows-amd64.exe" -OutFile tablam.exe
tablam.exe
```

### 3. Using Rust

**TablaM** is made with the [rust programming language](https://www.rust-lang.org). You need to [install it](https://www.rust-lang.org/tools/install).

Then download the code from [github](https://github.com/Tablam/TablaM). In the *root* of the folder you download it, run:

```bash
cargo run 
```

That is!

Now go to the [tutorial](/tutorial).