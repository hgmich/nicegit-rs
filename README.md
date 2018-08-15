# nicegit-rs

A collection of common higher-level abstractions over [git2-rs][git2-rs-repo].

# WARNING!
I published this crate to try to make sure I collect and extract some common and useful composed
Git operations into a single crate that can be reused in many projects, but for now this crate is
only intended be for personal use. As such, the API may break between minor releases - I am starting
from `0.0.1` to indicate this. I strongly recommend you don't rely on this crate for now.

# Usage

## Installing

If you're already using git2, you can just replace the `git2` crate in your `Cargo.toml`
with `nicegit`, and replace your `extern crate git2;` line with `extern crate nicegit;`. We
export a pinned copy of git2 to ensure that the two libraries are always in lock-step.

If you are not already using git2, ensure you read all the dependencies that are needed in the
[git2-rs][git2-rs-repo] README before adding this crate to your project.

## Using
The library is currently mainly shipped as a set of extension traits on existing `git2` types; just import
the `git2` types you need, `use` the extension traits for those types when you need them, and they'll appear  
on the normal `git2` types as methods.

You'll want to refer to the `git2` docs as well as these.

# License

Since this project is by definition a derivative work of the `git2` crate, it is licensed under
the same terms which is either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in git2-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[git2-rs-repo]: https://github.com/alexcrichton/git2-rs
