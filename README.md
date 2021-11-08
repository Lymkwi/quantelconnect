# QuanTel Connect

A Rust command line utility tool to connect to the QuanticTelecom captive portal.

## Usage

So far, the only mode supported so far requires providing password and login
in the command line (yikes) as follows
```
./quantelconnect -u email@example.com -p mypassword1sSh!t
```

When you are connecting on a new ethernet port, Quantic will ask you to confirm that you do indeed want
to connect on that port and disconnect every other port. The "force" parameter can be used to do so
```
./quantelconnect -u email@example.com -p mypassword1sSh!t -f
```

## TODO

The following features will *eventually* be added
 - [ ] Ability to load configuration from file

## Dependency

This program uses [`reqwest`](https://lib.rs/crates/reqwest) to handle all of
the network interaction, [`clap`](https://lib.rs/crates/clap) to parse and
access command line arguments, and [`kuchiki`](https://lib.rs/crates/kuchiki) to parse and navigate HTML source code.

## License

I'm releasing this as `CC0` because I couldn't care less what people do with my
dirty hack scripts but if for some incalculable reason you are a corporation or
large for-profit using this I hope my code makes your infrastructure
spontaneously combust.
