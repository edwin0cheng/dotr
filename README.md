# dotr , a dot files management tools

## Description

PLEASE DO NOT USE IT

This is a dot files management tools, which is using git as the backend.
It is still under development, and not ready for use.

## How it works

It will create a git repository in the directory `~/.local/share/dotr`, and use it to store all the dot files.
When you add a file to dotr, it will create a copy to the file in the git repository, and commit it.
When you push the changes, it will push all the changes to the git server.
When you pull the changes, it will pull all the changes from the git server, and copy the files to the original location.

It also support ignore files, which is useful when using in different operating systems.

## Installation

Clone the repository and navigate into the directory:

```bash
git clone https://github.com/edwin0cheng/dotr.git
cd dotr
```

Then, install the project using Cargo:

```bash
cargo install --path .
```

## Usage

```bash
dotr
```

You can use the following commands:

* `init`: init database for storage, and create a git repository
* `add`: Add a file to sync
* `push`: Commit all changes and push all changes to the git server
* `pull`: Pull all changes from the git server, and copy the files to the original location

## License

MIT