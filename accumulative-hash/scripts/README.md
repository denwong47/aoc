## Constant Generation

To generate the mixing constants used in the accumulative hash implementations, run the following script:

```bash
python3 scripts/generate_constants.py --bits <bit_size>
```

Replace `<bit_size>` with the desired bit size (e.g., `8`, `16`, `32`, `64`, `128`).

By default, this will be in form of a JSON for further processing. To directly generate Rust code, add the `--output` flag:

```bash
python3 scripts/generate_constants.py --bits <bit_size> --output rust
```
