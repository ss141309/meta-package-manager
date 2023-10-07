# genpack
A generic package manager for interfacing with multiple distro/platform specific package managers.

## Bsic Usage
```rust
use genpack::{managers, Package, PackageManager, Operation};

fn main() {
    let brew = managers::HomeBrew; // no constructor is called because it's a unit struct

    // Important: running any commands through the package manager if it is not in path/not installed
    // will result in a panic. See advanced usage for safely constructing verified instances.
    
    // single package operation (blocking call)
    brew.install("mypackage".into());
    brew.install(Package::from("packwithver").with_version("1.0.0"));

    // most methods return `ExitStatus` which can be used to check if
    // the operation was successful
    if brew.update_all().success() {
        println!("All packages updated/upgraded");
    }

    // multi pacakge operation (blocking call)
    brew.exec_op(
        &["mypackage".into(), "packwithver".into()],
        Operation::Uninstall,
    );

    // get packages matching search string
    for p in brew.search("python") {
        println!("{p}");
    }

    // list installed packages
    for p in brew.list_installed() {
        println!("{p}");
    }
```

## Advanced usage
```rust

use genpack::{managers, verify::Verify, Cmd, Commands, PackageManager};

fn main() {
    // creating a verified instance (package manager known to be in path/installed)
    // requires enabling the feature `verify`
    let Some(verified) = managers::Chocolatey.verify() else {
        return println!("HomeBrew not in path / not installed");
    };

    // get output by manually executing package manager commands (blocking call)
    let cmds = verified.consolidated(Cmd::Install, &["mypacakge"]); // gets appropriate Install command and flags
    let _output = verified.exec_cmds(&cmds);

    // get handle to child process (non-blocking)
    let cmds = verified.consolidated(Cmd::Update, &["some", "packages", "--quiet"]); // flags can also be included
    let _handle = verified.exec_cmds_spawn(&cmds);

    // fully customize commands with the general purpose `consolidated_args` fn
    // this example is impractical, but it shows how you can mix custom commands with default ones
    // default command is retrieved for `List` and default flags for `Install`
    let cmds = genpack::consolidate_args(
        verified.command(Cmd::List),
        &["anything"],
        verified.flags(Cmd::Install),
    );
    let _status = verified.exec_cmds_status(&cmds); // blocking call returns ExitStatus
}
  
```