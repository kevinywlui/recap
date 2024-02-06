# recap
 A command-tool with first-class support for outputting capture groups from regex

 ## Usage

 ```
 # foo.txt
host1 <1.2.3.4>
host2 <9.8.7.6>
 ```

 ```
 $ cat foo.txt | recap '<[.0-9]>'
 1.2.3.4
 9.8.7.6
 ```

 ## Why?

* Isn't this the same as `pcregrep -o`? Yes.
* Can't this be done with `awk`, `sed`, etc? Yes.

This is an excuse for me to learn some Rust by building a tool for a commonly used task that I do.

## Development

```
$ cargo build
```