# Halo2-ip

This project is built upon two repositories:

- [halo2-graph](https://github.com/pckennethma/halo2-graph)
- [halo2-scaffold](https://github.com/axiom-crypto/halo2-scaffold)

On top of these, this project implements two examples: **pmean** and **entropy**.

## Prerequisites

Before running the examples in this project, you need to have the **Rust environment** installed. If you haven't installed Rust yet, you can follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to set it up.

Once Rust is installed, you can proceed with running the examples.

## Running the Examples

The project runs in four main steps: **mock**, **keygen**, **prove**, and **verify**. Below is the command to run the **entropy** example:
```bash
LOOKUP_BITS=8 cargo run --example entropy -- --name entropy -k 16 mock
```

In this command:
- **LOOKUP_BITS**: You can adjust the number of lookup bits.
- **cargo run --example entropy**: This will run the `entropy` example.
- **--name entropy**: Specifies the name of the example to run.
- **mock**: The first step in the process, which you can replace with `keygen`, `prove`, or `verify` depending on the phase you want to run.

### Data Format

The data used in this project is located in the `data` folder, specifically the `in` folder. Please refer to the files inside this folder for the data format used in the examples.

## Adding New Examples

If you want to implement additional examples, you can refer to the [halo2-scaffold README](https://github.com/axiom-crypto/halo2-scaffold.git) for more information on how to extend the project with new examples.
