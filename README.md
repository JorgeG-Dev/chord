# Chord

- [About](#about)
- [Usage](#usage)
- [AI Usage](#ai-usage)

## About

This project is a lightweight `git` workspace management tool, meant to be used
for projects that are made up of multiple repos. Instead of using the native
`git` submodules, this tool provides an easier way of declaring and pinning 
external repo dependencies via manifest files.

This project was mainly started as a way to learn rust. It is inspired by a few
other existing tools, namely:

1. Zephyr's `west` tool
2. Google's `repo` tool
3. `gitman`

These all seemed to be written in Python, so I figured writing my own version
in rust would be a good way to learn the language since it seems relatively
straightforward enough. There's existing projects that can serve as a
reference and point of comparison. Main improvements `chord` has compared to
the aforementioned ones are the following:

1. *No dependencies*
    - Aforementioned tools require Python, Chord would be just the single
    binary.
2. *Purpose Built*, 
    - `west` includes a lot more functionality related to building embedded C
        projects, the repo mangement functionality is in addition to that.
    - `repo` seems to have been targeted for Android development, although
        there's nothing stopping someone from using it for other projects.
    - `gitman` ¯\\_(ツ)_/¯

## Install

To install, run the following command:
```
cargo install chord-ws
```

## Usage
```sh
Usage: chord <COMMAND>

Commands:
  init    Initializes the chord manifest directory and file
  status  Checks the status of the chord workspace against the manifest
  topdir  Prints the chord workspace root
  sync    Clones missing repos, fetches, and checks out to whatever is in the lockfile, defaults to chord manifest if there is no lockfile provided
  update  Performs same operations as sync, key difference being that it uses the manifest, regardless of whether there's a lockfile or not
  forall  Runs a user provided command in each repo in the chord workspace
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Quick Start

The following sequence outlines how to get started with chord:
```sh
chord init
# Modify generated manifest to suit your needs
chord sync
```
At this point, the chord workspace will be initialized and you'll have 
repos cloned locally and checked out to the specified revision. A lockfile
is also generated as a way to pin repo revisions. It can and should be version
controlled to guarantee chord workspaces can be reproduced across hosts.

### Manifest

The manifest file is where the user specifies which repos make up the chord
workspace. It is a YAML file named `chord.yaml` that serves as the source of
truth for the workspace. The manifest file also serves as the marker for the 
workspace's top directory. The following is what gets generated when `chord init`
is run:

```yaml
repos:
- remote: https://github.com/JorgeG-Dev/nix-templates.git
  name: chord 
  revision: main 
  location: "."
```

Here's a breakdown of the fields:

- repos: The repos that make up the chord workspace.
- remote: Where to find the repo.
- name: The name the repo should appear under in the workspace.
- location: Where the repo should be cloned, relative to the workspace top directory
(optional, defaults to chord workspace root).

Repos will be cloned into `<workspace-top-directory>/<location>/<name>`. Assuming the 
repo name is `test`, the chord workspace top directory is `/home/here`, and the location
is `deps`, the repo specified by the URL will be cloned to `/home/here/deps/test`

### Lockfile

Whenever a `sync` or `update` command is run, a lockfile is generated. The lockfile
specifies the commit hash each repo should be checked out to. It evaluates branches
and tags to the absolute hash and pins them to that revision. The name of the
lockfile is always `chord.lock.yaml` and looks like this:

```yaml
chord: <commit hash>
```

Assuming the init manifest is used. 

## AI Usage

The intent of this project was to learn how to write Rust. With that being the 
case, I tried to keep AI usage down to a minimum in the actual application code. 
The following locations are where AI was used to generate code:

1. `libgit2.rs`
    - Specifically, the `rev_as_hash` trait function implementation. I had a messy
    implementation that I figured Claude could refactor into something cleaner, that
    is what it came up with. No way did I know how to properly use `.map` and
    `.and_then`.
2. `/tests` 
    - I wrote the tests for `sync.rs` and `update.rs` manually, as well
    as setting up the `common.rs` module. Once that was set up, I just fed those
    files into Claude and it generated tests for the other command files too.

3. `manifest.rs`, `workspace.rs`, `utils.rs`
    - Similar to `2`, I had Claude generate the test modules for these.

The code has been tested manually and I went over the the code Claude generated to
make sure it made sense. Can't say it means anything given the lack of Rust knowledge
but it's better than nothing. Beyond this, AI was mainly used as a smarter search 
engine to point me in the right direction for existing crates like `git2`, `comfy_table`, etc.
