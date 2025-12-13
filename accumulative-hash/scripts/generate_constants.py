#!/usr/bin/env python3
"""
Script to generate constant values for accumulative hashing traits.
"""
import sys
import inspect
import itertools
import json
import random
from decimal import Decimal, getcontext
from typing import Protocol, TYPE_CHECKING

import argparse

RNG_SEED = 42
random.seed(RNG_SEED)

# Set decimal precision high enough for our calculations
getcontext().prec = 100

if TYPE_CHECKING:
    from typing import Generator, TypedDict, Literal

    class ConstantSet(TypedDict, total=True):
        """
        The return type for constant set generation.
        """

        bits: int
        seed: int
        shiftConstants: list[int]
        multiplierConstants: list[int]


def prime_factory() -> "Generator[int, None, None]":
    """
    Simple, inefficient prime number generator.

    Realistically for our purpose, we are only generating a few of them
    so efficiency is not a concern.

    Examples
    --------
    Generate the first 10 primes:

        >>> primes = prime_factory()
        >>> [next(primes) for _ in range(10)]
        [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]

    """
    found = [2]

    yield found[-1]
    candidate = found[-1]

    while True:
        candidate += 1

        for prime in found:
            if prime * prime > candidate:
                found.append(candidate)
                yield candidate
                break

            if candidate % prime == 0:
                break


class ParsedArgs(Protocol):
    mul_count: int
    bits: int
    output: "Literal['json', 'rust']"


def parse_args() -> ParsedArgs:
    parser = argparse.ArgumentParser(
        description="Generate constant values for accumulative hashing traits."
    )
    parser.add_argument(
        "--shift-count",
        type=int,
        default=3,
        help="Number of shift constants to generate (default: 3)",
    )
    parser.add_argument(
        "--bits",
        type=int,
        default=64,
        help="Bit size of the target integer type (default: 64)",
    )
    parser.add_argument(
        "--output",
        type=str,
        choices=["json", "rust"],
        default="json",
        help="Output format (default: json)",
    )

    return parser.parse_args()


def golden_ratio_constant(n: int) -> int:
    """
    Return ``floor(2^n / φ) = floor(2^n * (sqrt(5) - 1) / 2)``
    where ``φ`` is the golden ratio.

    Valid for n up to at least 128 bits; beyond that, precision issues may arise,
    further testing needed.

    This is typically a non-issue since C does not support integer types larger than 128
    bits.
    """
    if not (0 <= n <= 128):
        raise ValueError("n must be between 0 and 128")

    sqrt5 = Decimal(5).sqrt()
    inv_phi = (sqrt5 - 1) / 2

    return int((Decimal(2) ** n) * inv_phi)


def constant_from_prime(prime: int, bits: int) -> int:
    """
    Generate a constant from a prime number suitable for accumulative hashing.

    The constant is derived by the first ``N`` bits of the fractional parts of the
    square roots of the primes.

    Parameters
    ----------
    prime : int
        The prime number to derive the constant from.

    bits : int
        The bit size of the target integer type.

    Returns
    -------
    int
        The generated constant.
    """
    sqrt_prime = Decimal(prime).sqrt()
    fractional_part = sqrt_prime - int(sqrt_prime)
    constant = int(fractional_part * (Decimal(2) ** bits))
    return constant


def generate_constants(
    bits: int,
) -> "Generator[int, None, None]":
    """
    Generate constants for multipliers. These multipliers must be odd numbers,
    so we skip primes that would yield even constants.

    Parameters
    ----------
    bits : int
        The bit size of the target integer type.

    Yields
    ------
    int
        The next constant suitable for use as a multiplier.
    """

    primes = prime_factory()
    while True:
        prime = next(primes)
        constant = constant_from_prime(prime, bits)
        if constant % 2 == 1:
            yield constant


def generate_shifts(
    bits: int,
    shift_count: int,
) -> "Generator[int, None, None]":
    """
    Generate shift constants based on the golden ratio.

    Parameters
    ----------
    bits : int
        The bit size of the target integer type.

    Yields
    ------
    int
        The next shift constant.
    """
    primes = prime_factory()
    # Don't over shift -- limit to bits // 4
    random_shifts = [next(primes) % (bits // 2) for _ in range(shift_count)]
    random.shuffle(random_shifts)

    final_shifts = itertools.cycle(dict.fromkeys(random_shifts, None))

    for _, shift in zip(range(shift_count), final_shifts):
        yield bits // 2 - shift


def display_as_hex(value: int) -> str:
    """
    Display an integer value as a hexadecimal string.

    Parameters
    ----------
    value : int
        The integer value to convert.

    Returns
    -------
    str
        The hexadecimal representation of the integer.
    """
    hex = f"{value:x}".upper()
    return f"0x{hex}"


def generate_constant_set(
    shift_count: int,
    bits: int,
) -> "ConstantSet":
    """
    Generate a set of constants for accumulative hashing.

    Parameters
    ----------
    shift_count : int
        The number of shift constants to generate.

        The multiplier constants will be ``shift_count - 1``.

    bits : int
        The bit size of the target integer type.

    Returns
    -------
    ConstantSet
        The generated set of constants.
    """
    seed = golden_ratio_constant(bits)
    shifts = generate_shifts(bits, shift_count)
    multipliers = generate_constants(bits)

    return {
        "bits": bits,
        "seed": display_as_hex(seed),
        "shiftConstants": [next(shifts) for _ in range(shift_count)],
        "multiplierConstants": [
            display_as_hex(next(multipliers)) for _ in range(shift_count - 1)
        ],
    }


def print_rust(constant_set: "ConstantSet") -> str:
    """
    Print the constant set in Rust format.

    Parameters
    ----------
    constant_set : ConstantSet
        The constant set to print.
    """
    if constant_set["bits"] not in (8, 16, 32, 64, 128):
        print(
            f"\x1b[33m\x1b[1mWarning\x1b[22m: ported bit size \x1b[1mu{constant_set['bits']}\x1b[22m for Rust output.\x1b[0m",
            file=sys.stderr,
        )

    template = inspect.cleandoc(
        """
        /// Implementation of [`IsAccumulativeHashType`] for [`u{bits}`].
        /// 
        /// This implementation uses constants generated by the script
        /// ``scripts/generate_constants.py --bits {bits} --output rust```.
        impl IsAccumulativeHashType for u{bits} {{
            const SEED: Self = {seed};
            const SHIFT_CONSTANTS: [Self; {shift_count}] = [{shifts}];
            const MULTIPLIER_CONSTANTS: [Self; {multiplier_count}] = [{multipliers}];
        }}
        """
    )

    return template.format(
        bits=constant_set["bits"],
        seed=constant_set["seed"],
        shift_count=len(constant_set["shiftConstants"]),
        shifts=", ".join(str(shift) for shift in constant_set["shiftConstants"]),
        multiplier_count=len(constant_set["multiplierConstants"]),
        multipliers=", ".join(mult for mult in constant_set["multiplierConstants"]),
    )


if __name__ == "__main__":
    args = parse_args()

    if args.shift_count < 2:
        raise ValueError("shift-count must be at least 2")

    if args.bits > 128:
        raise ValueError("bits must be at most 128")

    constant_set = generate_constant_set(
        shift_count=args.shift_count,
        bits=args.bits,
    )

    if args.output == "rust":
        print(print_rust(constant_set))
    else:
        print(json.dumps(constant_set, indent=2))
