# LeetCode Rust contribution guidelines

Thank you for your interest in making this project
better! We'd love to have your contribution. We expect all contributors to
abide by the [Rust code of conduct], which you can find at that link or in the
[`CODE_OF_CONDUCT.md`] file in this repository.

[rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
[`code_of_conduct.md`]: CODE_OF_CONDUCT.md

## License

This project is dual licenced under the MIT and Apache 2.0 licenses, and so are all
contributions. Please see the [`LICENSE-MIT`] and [`LICENSE-APACHE`] files in
this directory for more details.

[`license-mit`]: LICENSE-MIT
[`license-apache`]: LICENSE-APACHE

## Pull Requests

To make changes to this project, please send in pull requests on GitHub to the `master`
branch. We'll review them and either merge or request changes. Github CI tests
everything as well, so you may get feedback from it too.

If you make additions or other changes to a pull request, feel free to either amend
previous commits or only add new ones, however you prefer. We may ask you to squash
your commits before merging, depending.

## Issue Tracker

You can find the issue tracker [on
GitHub](https://github.com/shekohex/leetcode-rs/issues). If you've found a
problem, please open an issue there.

We use the following labels:

- `enhancement`: This is for any request for improve any submitted solutions.
- `discussion`: A discussion about improving/solving problems ; this may lead to new
  enhancement.
- `P-Easy`: This refers to an easy to solve problem.
- `P-Medium`: This refers to a medium to solve problem.
- `P-Hard`: This refers to a hard to solve problem.

## Contributing workflow

1. [install Rust].
2. Fork this project to your github account.
3. then clone it localy.

```bash
$ git clone https://github.com/{YOUR_GITHUB_USERNAME}/leetcode-rs
$ cd leetcode-rs
```

4. Pick one of Leetcode problems [here].
5. Check if there is any Issues/PRs for that problem or not. (if not then continue)
6. Create a new git branch with problem name. e.g:

```bash
$ git checkout -b two-sum
```

7. Make a new Cargo project (as a library) named after problem name in one of (easy, medium, hard) directory depending on the level of the picked problem. e.g

```bash
$ cargo new --lib ./easy/two-sum
```

8. Add your project to the Cargo Workspace in [Cargo.toml](Cargo.toml) file
9. Start writing your code.
10. Run tests localy. e.g:

```bash
$ cargo test -p two-sum
```

11. Add your solved problem list in the `README.md` file located in one of (easy, medium, hard) directory depending on the level of this problem. e.g: in [easy/README.md](easy/README.md)

```md
- [Two Sum](https://leetcode.com/problems/two-sum/) See #1
```

11. Commit your changes once you are happy with it.
12. Create a new PR with your changes to the `master` branch.
13. Thank you :heart:

[install rust]: http://rust-lang.org/install.html
[here]: https://leetcode.com/problemset/all/
