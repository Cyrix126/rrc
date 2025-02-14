# Name

## Status of developmenent

**Work in Progress**

## About

CLI client using the [rcc library] to interact with Rakuten as a seller

## Objective

Offer a way to manage products and orders through a CLI with various helpers.

## Features

### Basic features

- [ ] retrieve all products
- [ ] create/update products from a csv file
- [ ] overwrite all products with a csv file
- [ ] confirm/decline an order

### Helpers

- [ ] get the products with selected conditions
- [ ] apply global change to products retrieved
- [ ] get the stock of a product
- [ ] with a reference, update the stock
- [ ] find products on Rakuten based on title
- [ ] find a product on Rakuten based on a barcode

## Installation

## Usage

1. Put your token and username in the configuration file
```bash
$ cat ~/.config/rcc/config.toml
[authentication]
username = "your_username"
token = "your_token"
```
2. Get the help usage from the binary
```bash
$ rcc --help
```

## Example

## Bug Reporting

Open an [issue](https:://github.com/Cyrix126/rrcc/issues) using the CLI client bug report model.

## Contributing
## Security
## Documentation
## License

GNU GPL v3
