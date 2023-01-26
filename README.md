# Libwrc-rc
**Unsafe Rust bindings for libwrc.**

## Installation

1. add in Cargo.toml 
```toml
[dependencies]
libwrc = "*"
```

## Usage

```rs
extern crate cc;
```

## Example

```rs
fn main() {
    let cap: mut wrc;
    let ifc: wc_iflist = wrc_get_interfaces();
    
    wrc_default(cap);

    let opts = wrc_setopts(&cap, ifc.ifc[2], PA_NULL, 0);
    if opts != 0 {
        print!("setopts error\n");
        exit(0);
    }

    wrc_cap(cap, 1, DEFAULT_CAP);
    
    wrc_destroy(cap);
}
```

## Development

```bash
$ cargo test
```

## Contributing

1. Fork it (<https://github.com/wirecroc/libwrc-rs/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Contributors

- [Nikolaycc](https://github.com/nikolaycc) - creator and maintainer
- [PabloEscobar1337](https://github.com/PabloEscobar1337) - creator and maintainer
