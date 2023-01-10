# Contributing to assemble
All contributions to this repository start as an issue. Issues should propose changes by identifying
bugs or possible new features. I am interesting in not just improving this repositories code base,
but also my knowledge and abilities as a maintainer, so feel free to recommend changes to this
document as well!

## Recommended Reading
- [github's contributing doc](https://github.com/github/docs/blob/main/CONTRIBUTING.md)

## Branch Naming
The name of the branch should be something like `<relevant_issue_number>-<short_description>`.
For example `2-fix-readme`. Text should be ALL lower-case.

I would recommend just having github automatically create the branch name for you! When
looking at an issue there is a `Create a branch` link that will name the branch for you.
Copy the branch name, and checkout on your local fork.

## Conventional Commits
Read [this](https://www.conventionalcommits.org/en/v1.0.0/) to learn more about conventional
commits. The short of it is use `<keyword>: <short_description>` for _all_ of your commits.
I would encourage making smaller commits, as it is hard to have a short descriptiong describe
a large swath of changes.

The current accepted list of keywords are
- `fix:` --> A bug fix
- `feat:` --> A new feature
- `build:` --> Changes to how code is compiled/built
- `ci:` --> Changes to automated tooling, for example how test suit is run.
- `docs:` --> Changes to documentation
- `refactor:` --> A code chanage that doesn't change functionality
- `test:` --> Adding or correcting tests

PLEASE READ THIS: you can use your commit message to link issues <--> pull requests. The commit
message `feat: add cool feature, resolve #23` will automatically link the pull request with
issue `#23`. This will make everyone's life easier :).

### Commit Signing
I would recommend for your own safety to sign your commits.

# License
Any contributions you make will be under the GPL-2.0 license. Feel free to ask questions or read
the `LICENSE` doc.

# Coding Style
The is no currently enforced style guide beyond what standard rust tooling already has. There
are some tickets created to enforce some basic styling by use of `clippy`.

## Testing Code
This project is still in early development so standards for testing code are more laxed then they
will be later when the project is more stable. Please try to write testable code!
