# keytones: MIDI key to frequency or period
Bart Massey 2025 (version 0.2.0)

This crate implements functions that take a MIDI key number
(in the range 0 to 127 inclusive) and produce a note
frequency or period.

There are two versions of these routines: "exact" versions
with high precision and "approximate" versions. The
approximate versions are "accurate enough". They may be
slightly faster, and may take slightly less program memory —
neither of these has been tested, though.

The crate can be compiled `no_std` with
`--no-default-features`. Otherwise the `std` feature will be
used.

Full crate [rustdoc](https://bartmassey.github.io/keytones)
is available.

## Acknowledgements

Thanks to Per Gantelius (@stuffmatic on Github) for the
`microcheby` crate. I had previously hand-rolled a similar
solution, but theirs was cleaner and better.

## License

This work is made available under the "Apache 2.0 or MIT
License". See the file `LICENSE.txt` in this distribution for
license terms.
