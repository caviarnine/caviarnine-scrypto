# Unittests
need to install cargo nextest (google it) to run the unittests
cargo nextest run -r --retries 3

single:
cargo nextest run -r --retries 3 --test test_roles

Or

scrypto tests