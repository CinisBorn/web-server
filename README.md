# Description 

A web server written in Rust for learning purposes.

The project is still under active development; however, it is not production-ready.

This project depends on several crates developed by me. The goal is to build a web server from scratch. It currently includes a parser, router, connection layer, thread pool for concurrency, and HTTP core utilities.

The parser, router, and connection components are now standalone crates. You can find their repositories here:
[parser](https://github.com/CinisBorn/http-parser)
[router](https://github.com/CinisBorn/http-router)
[tcp-connection](https://github.com/CinisBorn/tcp-connection)
