# match_any

[![license-image][]] [license-file]
[![travis-ci-image][]] [travis-ci-link]
[![appveyor-ci-image][]] [appveyor-ci-link]
[![coveralls-image][]] [coveralls-link]

## Overview

Match like macro for std::any::Any.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
match_any = { git = "https://github.com/DarkEld3r/match_any.git" }
```

and this to your crate root:

```rust
extern crate match_any;
```

### Example

```rust
#[macro_use] extern crate match_any;

fn main() {
    match_any!(make_any(10) => 
        _x: i8 => { 
            // ...
        }
        _x: i16 => { 
            // ...
        }
        // ...
    );
}
```

## License

Match_any is licensed under the MIT license (see the `LICENSE` file).

[travis-ci-image]: https://travis-ci.org/DarkEld3r/match_any.png?branch=master
[travis-ci-link]: https://travis-ci.org/DarkEld3r/match_any
[appveyor-ci-image]: https://ci.appveyor.com/api/projects/status/jp11knq37hosf529/branch/master?svg=true
[appveyor-ci-link]: https://ci.appveyor.com/project/DarkEld3r/match-any
[license-image]: http://img.shields.io/badge/license-MIT-blue.svg
[license-file]: https://github.com/DarkEld3r/match_any/blob/master/LICENSE
[coveralls-image]: https://coveralls.io/repos/github/DarkEld3r/match_any/badge.svg?branch=master
[coveralls-link]: https://coveralls.io/github/DarkEld3r/match_any?branch=master
