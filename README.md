# touch
A Rust implementation of the touch command (for Windows, mostly...). It's not trying to replace `touch` from a Unix or Linux system, but just brings the functionality (and familiarity) to the Windows PowerShell.

## Installation
- Install Rust: see [steps](https://www.rust-lang.org/tools/install) for Windows
- Make sure it is setup correctly with the following command in your PowerShell: `rustc --version`
- Clone the repository to the location of your choice:
```powershell
PS> git clone git@github.com:srbdev/touch.git && cd touch
PS> cargo install --path .
```
- Open a new PowerShell terminal, and check that it installed correctly:
```powershell
PS> touch -h
```
