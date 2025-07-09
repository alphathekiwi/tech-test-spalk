## Running the CLI

If you do not already have Rust installed, you can download it from.

[https://rustup.rs/]()

The installation should be straightforward and easy especially if you already have LLVM installed for C/C++.
More information can be found here.

[https://rust-lang.github.io/rustup/installation/other.html]()

If you prefer not to install Rust, see the [prebuilt binaries](https://github.com/spalk-tech/spalk-tech-test/releases).

The commands I have been using to test it with my directory structure are:
```bash
cargo build
cat tech-test-spalk/test_failure.ts | target/debug/spalk-tech-test.exe
cat tech-test-spalk/test_success.ts | target/debug/spalk-tech-test.exe
```

Finally failure file, success file and incomplete first packet are all built into test cases which can be run using cargo.
This greatly simplified development and allowed my to prototype swiftly and efficiently while catching any regressions, bugs or feature holes.
```bash
cargo test
```

#### Thoughts going into this.

The MPEG-TS packet parser in this situation is running as a CLI tool to which data is piped via stdin so its important that.

* The CLI should be relatively performant.
* The CLI should be easy to run and easy to test.
* The CLI will require bit masking and (byte, byte) to u16 type conversion for the PID.

##### Languages considered:

* Rust - This is my first choice since it meets all the requirements and I am confident in using it.
* Python - I do like prototyping software in Python and did consider using it for its portability.
* JS/TypeScript - Since this CLI is not intended to run on the frontend I out of personal preference opted not to use JS or TypeScript.

##### Implementation details:

* Function main() will handle the CLI arguments and reading from stdin.
* Function parse_packets() will take the bytes and parse them into MPEG-TS packets escalating any errors back to the caller.
* Testing: parse_packets should also be testable with an open file handle so as to be able to automate passing a file to it

There is probably no need to separate the code further as ultimately it should be succinct enough to be easily understood.

##### Further thoughts:

* Additional testing could be done if time permits...
* Dependencies have been kept minimal, with only 29 total dependencies allowing for rapid compilation and execution.
