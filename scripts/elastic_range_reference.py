"""Check elastic boundary reference bits with rational IEEE nearest-even rounding.

Run: python3 scripts/elastic_range_reference.py
Columns: width, lambda bits, mu bits, E bits, K bits. No host float arithmetic.
"""
from fractions import Fraction
from pathlib import Path


def format_spec(width):
    """Precision and minimum/maximum normal binary exponents."""
    return (24, -126, 127) if width == 32 else (53, -1022, 1023)


def exact(bits, width):
    """Interpret finite native bits as a rational; special values return None."""
    precision, minimum, _ = format_spec(width)
    mantissa_bits = precision - 1
    exponent = (bits >> mantissa_bits) & ((1 << (width - precision)) - 1)
    mantissa = bits & ((1 << mantissa_bits) - 1)
    if exponent == (1 << (width - precision)) - 1:
        return None
    significand = mantissa + (0 if exponent == 0 else 1 << mantissa_bits)
    power = (minimum if exponent == 0 else exponent + minimum - 1) - mantissa_bits
    value = Fraction(significand) * Fraction(2) ** power
    return -value if bits >> (width - 1) else value


def nearest(value, width):
    """Integer quotient/remainder rounding, including the overflow midpoint."""
    precision, minimum, maximum = format_spec(width)
    sign = 1 << (width - 1) if value < 0 else 0
    value = abs(value)
    if value == 0:
        return sign
    exponent = value.numerator.bit_length() - value.denominator.bit_length()
    if value < Fraction(2) ** exponent:
        exponent -= 1
    quantum = max(exponent - (precision - 1), minimum - (precision - 1))
    scaled = value / Fraction(2) ** quantum
    significand, remainder = divmod(scaled.numerator, scaled.denominator)
    if 2 * remainder > scaled.denominator or (
        2 * remainder == scaled.denominator and significand & 1
    ):
        significand += 1
    if significand == 0:
        return sign
    if significand.bit_length() > precision:
        significand //= 2
        quantum += 1
    result_exponent = quantum + significand.bit_length() - 1
    if result_exponent > maximum:
        return sign + (((1 << (width - precision)) - 1) << (precision - 1))
    if result_exponent < minimum:
        return sign + significand
    return (
        sign
        + ((result_exponent - minimum + 1) << (precision - 1))
        + significand - (1 << (precision - 1))
    )


def check_fixture():
    fixture = Path(__file__).resolve().parent.parent / "tests/elastic_range/overflow_cases.txt"
    count = 0
    midpoint_counts = {32: 0, 64: 0}
    for line in fixture.read_text().splitlines():
        width, lam, shear, young, bulk = line.split()
        width = int(width)
        lam, shear = exact(int(lam, 16), width), exact(int(shear, 16), width)
        assert lam is not None and shear is not None and shear > 0
        assert 3 * lam + 2 * shear > 0, line
        expected_young = nearest(shear * (3 * lam + 2 * shear) / (lam + shear), width)
        exact_bulk = lam + Fraction(2, 3) * shear
        expected_bulk = nearest(exact_bulk, width)
        assert expected_young == int(young, 16) and expected_bulk == int(bulk, 16), line
        precision, _, maximum = format_spec(width)
        midpoint = 2 * Fraction(2) ** maximum - Fraction(2) ** (maximum - precision)
        midpoint_counts[width] += exact_bulk == midpoint
        count += 1
    assert all(midpoint_counts.values()), midpoint_counts
    print(f"{count} exact-rational E/K cases; exact bulk overflow ties: {midpoint_counts}")


if __name__ == "__main__":
    check_fixture()
