# Exegr


Trying to optimize code that relied on python's regex brought me to a point
where I was still looking at ~400 (at times unicode rich) patterns and 10 sentences
to search in. This search cost the algorithm over 100ms. It came to a point where it seemed there
is no other way than to shed some code which meant a sacrifice of accuracy over speed.

I read about the problems with the [implementation](https://swtch.com/~rsc/regexp/regexp1.html) and better
alternatives to `regex` libraries. 
I found [re2](https://github.com/google/re2) and its [python wrapper](https://github.com/facebook/pyre2/) the
lack of features, change in APIs throughout the codebase meant having to keep this as a step
to try later.

I decided to rewrite the particular function in Rust and port it back to Python
using [pyo3](https://github.com/PyO3/pyo3). The results have been dramatically [huge](https://ltbringer.github.io/blog/regular-expressions-and-efficiency#compile-time-speed-boost).

This might not be helpful for general purpose usage due to poor abstraction for the sake of quick testing.


## Installation
You can easily add exegr to your python projects like:
```
pip install exegr
```

For testing and development use the following:

```shell
cargo build --release
cp target/libexegr.so /path/to/py/project
```

> On windows and linux, you can build normally with cargo build --release. On Mac Os, you need to set additional linker arguments. The simplest solution is to create a .cargo/config with the following content:

```
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

## Usage
```python
from exegr import GroupRegexMatch

patterns: List[str] = []
sentences: List[str] = []
# load patterns
...
group_regex_match = GroupRegexMatch(patterns)

"""
The sentences passes to this function
are expected to be ASR output which provide
multiple options. If your input is a raw string
use sentences = [sentence] instead.
"""
group_regex_match(sentences)
```
