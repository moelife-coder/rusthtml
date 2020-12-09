# RustHtml - a small html parser

<!-- ABOUT THE PROJECT -->
## About The Project

`RustHtml` is a small html parser written in rust.

> Warning: this library is not production-ready. Many of the tags and standard are not implemented (mainly located in `tag_optimize()`). PRs are welcomed.

Completed & planned features:

- [x] Parse simple html
- [x] Parse html with void elements
- [x] Parse html with javascript
- [ ] Parse html with complicated elements (such as html without `head` ending tag, etc)

## Benchmarking

Run `cargo bench` to benchmark the program.

On my local device, parsing a `43833` lines html requires `18.697 ms` to complete

<!-- LICENSE -->
## License

Distributed under the GPL-3.0-Only License. See `LICENSE` for more information.
