# Coffee

Coffee is a custom implementation of the original Cobalt Strike's [beacon_inline_execute](https://hstechdocs.helpsystems.com/manuals/cobaltstrike/current/userguide/content/topics_aggressor-scripts/as-resources_functions.htm#beacon_inline_execute). It is written in Rust and supports most of the features of the Cobalt Strike compatibility layer. Coffee is structured so it can be used as a library in other projects too.

The original blog post can be found here: <https://labs.hakaioffsec.com/coffee-a-coff-loader-made-in-rust/>

## Usage

```bash
$ coffee.exe -h
Coffee: A COFF loader made in Rust

Usage: coffee.exe [OPTIONS] --bof-path <BOF_PATH> [-- <ARGS>...]

Arguments:
  [ARGS]...  Arguments to the BOF passed after the "--" delimiter, supported types are: str, wstr, int, short

Options:
  -b, --bof-path <BOF_PATH>      Path to the Beacon Object File (BOF)
  -e, --entrypoint <ENTRYPOINT>  The entrypoint name to execute in case of a custom entrypoint name [default: go]
  -v, --verbosity <VERBOSITY>    Verbosity level, 0 = ERROR, 1 = WARN, 2 = INFO, 3 = DEBUG, 4 = TRACE [default: 0]
  -h, --help                     Print help
  -V, --version                  Print version
```

### Arguments

Arguments for the BOF can be passed after the `--` delimiter. Each argument must be prefixed with the type of the argument followed by a colon (`:`). The following types are supported:

- `str` - A null-terminated string
- `wstr` - A wide null-terminated string
- `int` - A signed 32-bit integer
- `short` - A signed 16-bit integer

## Example

Using the `dir.x64.o` BOF from the [trustedsec/CS-Situational-Awareness-BOF](https://github.com/trustedsec/CS-Situational-Awareness-BOF) repository and passing arguments to the BOF:

```bash
coffee.exe --bof-path .\dir.x64.o -- wstr:"C:\\Windows\\System32"
```

## Usage as library

```bash
cargo add coffee-ldr
```

Coffee can be used as a library in other projects. The following example shows how to use Coffee to load a BOF and execute the BOF:

```rust
use coffee_ldr::loader::Coffee;

fn main() {
    let whoami_bof: [u8; 6771] = [
        0x64, 0x86, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x14, 0x00, 0x00, 0x33, 0x00, 0x00,
        ...
    ];

    let _ = Coffee::new(&whoami_bof).unwrap().execute(None, None, None);
}
```

The example above will execute the BOF passed as an array of bytes and show the output in console.

The detailed documentation can be found at: <https://docs.rs/coffee-ldr/latest/coffee_ldr/loader/struct.Coffee.html>

## Building from source

1. Install Rust from <https://rustup.rs/>
2. Clone the repository
3. Build the project using

```bash
cargo build --release
```

## License

Coffee is licensed under the GNU GPLv3 license. See [LICENSE](LICENSE) for more information.

## Contributing

Pull requests are welcome. Please open an issue first to discuss what you would like to change.

## References

Thanks to the amazing people who have written about COFF loaders and helped me understand the format:

- <https://github.com/trustedsec/COFFLoader>
- <https://github.com/Cracked5pider/CoffeeLdr>
- <https://github.com/yamakadi/ldr>
- <https://www.trustedsec.com/blog/coffloader-building-your-own-in-memory-loader-or-how-to-run-bofs/>
- <https://0xpat.github.io/Malware_development_part_8/>
- <https://otterhacker.github.io/Malware/CoffLoader.html>
- <https://signal-labs.com/trainings/offensive-tool-development/>
- <https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#coff-file-header-object-and-image>
- <https://blog.cloudflare.com/how-to-execute-an-object-file-part-1/>
