# Chord

- [About](#about)

## About

This project is a lightweight `git` workspace management tool, meant to be used
for projects that are made up of multiple repos. Instead of using the native
`git` submodules, this tool provides an easier way of declaring and pinning 
external repo dependencies via mainfest files.

This project was mainly started as a way to learn rust. It is inspired by a few
other existing tools, namely:

1. Zephyr's `west` tool
2. Google's `repo` tool
3. `gitman`

These all seemed to be written in Python, so I figured writing my own version
in rust would be a good way to learn the language since it seems relatively
straightforward enough and there's existing projects that can serve as a
reference and point of comparison. Main improvements `chord` has compared to
the aforementioned ones are the following:

1. *No dependencies*
    - Aforementioned tools require Python
2. *Purpose Built*, 
    - `west` includes a lot more functionality related to building embedded C
        projects, the repo mangement functionality is in addition to that
    - `repo` seems to have been targetting Android development, although
        there's nothing stopping someone from using it for other projects
    - `gitman` ¯\\_(ツ)_/¯

