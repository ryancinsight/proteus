"""Exact rational reference, with explicit nearest-even binary encoding."""
from fractions import Fraction
from pathlib import Path
import hashlib
import random


def power(exponent):
    return Fraction(2) ** exponent


def decode(bits, precision, maximum):
    fraction_bits = precision - 1
    fraction = bits & ((1 << fraction_bits) - 1)
    exponent = bits >> fraction_bits
    if exponent == 0:
        return fraction * power(1 - maximum - fraction_bits)
    return ((1 << fraction_bits) + fraction) * power(exponent - maximum - fraction_bits)


def encode(value, precision, maximum):
    if value == 0:
        return 0
    exponent = value.numerator.bit_length() - value.denominator.bit_length()
    if value < power(exponent):
        exponent -= 1
    minimum = 1 - maximum
    if exponent > maximum:
        return ((2 * maximum + 1) << (precision - 1))
    step = max(exponent, minimum) - (precision - 1)
    scaled = value / power(step)
    quotient, remainder = divmod(scaled.numerator, scaled.denominator)
    doubled = 2 * remainder
    quotient += doubled > scaled.denominator or (doubled == scaled.denominator and quotient % 2)
    if exponent < minimum:
        return quotient
    if quotient == 1 << precision:
        quotient >>= 1
        exponent += 1
    if exponent > maximum:
        return ((2 * maximum + 1) << (precision - 1))
    return ((exponent + maximum) << (precision - 1)) + quotient - (1 << (precision - 1))


def make_cases():
    rng = random.Random(0x50524F544855)
    rows = []
    for width, precision, maximum in [(32, 24, 127), (64, 53, 1023)]:
        minimum_subnormal = 1 - maximum - (precision - 1)
        infinity = (2 * maximum + 1) << (precision - 1)

        def add(kind, density, heat, conductivity):
            if not all(0 < value < infinity for value in [density, heat]) or not 0 <= conductivity < infinity:
                return
            r, c, k = (decode(x, precision, maximum) for x in [density, heat, conductivity])
            exact = k / (r * c)
            rows.append((width, kind, density, heat, conductivity, encode(exact, precision, maximum)))

        for kind, target, first, second in [
            ('overflow-midpoint', (2 - power(-precision)) * power(maximum),
             minimum_subnormal // 2 - 1, minimum_subnormal - (minimum_subnormal // 2 - 1) - 2),
            ('zero-midpoint', power(minimum_subnormal - 1), maximum // 2 + 1,
             maximum - (maximum // 2 + 1) + 2),
        ]:
            for _ in range(16):
                mantissas = [Fraction((1 << (precision - 1)) + rng.getrandbits(precision - 1), 1 << (precision - 1)) for _ in range(2)]
                density = encode(mantissas[0] * power(first), precision, maximum)
                heat = encode(mantissas[1] * power(second), precision, maximum)
                denominator = decode(density, precision, maximum) * decode(heat, precision, maximum)
                center = encode(target * denominator, precision, maximum)
                for offset in [-2, -1, 0, 1, 2]:
                    add(kind, density, heat, center + offset)

        for _ in range(32):
            values = [rng.randrange(1, infinity) for _ in range(3)]
            add('full-range', *values)
        for er, ec in [(minimum_subnormal // 2, minimum_subnormal // 2),
                       (maximum // 2 + 1, maximum // 2 + 1),
                       (1 - maximum, 0), (0, 1 - maximum), (0, 0)]:
            density = encode(power(er), precision, maximum)
            heat = encode(power(ec), precision, maximum)
            for ek in [minimum_subnormal, er, 0, maximum]:
                add('power-of-two', density, heat, encode(power(ek), precision, maximum))
        for bits in [1, (1 << (precision - 1)) - 1, 1 << (precision - 1)]:
            assert encode(decode(bits, precision, maximum), precision, maximum) == bits
        # Independent sequence preserves all prior424 inputs while extending
        # the oracle to final-range decisions with a normal capacity product.
        normal_rng = random.Random(0x4E4F524D414C + width)
        for kind, target, first, second in [
            ('normal-product-overflow', (2 - power(-precision)) * power(maximum), -1, -1),
            ('normal-product-zero', power(minimum_subnormal - 1), maximum // 2,
             maximum - maximum // 2 - 2),
        ]:
            for _ in range(16):
                mantissas = [Fraction((1 << (precision - 1)) + normal_rng.getrandbits(precision - 1), 1 << (precision - 1)) for _ in range(2)]
                density = encode(mantissas[0] * power(first), precision, maximum)
                heat = encode(mantissas[1] * power(second), precision, maximum)
                denominator = decode(density, precision, maximum) * decode(heat, precision, maximum)
                center = encode(target * denominator, precision, maximum)
                for offset in [-2, -1, 0, 1, 2]:
                    add(kind, density, heat, center + offset)
    return rows


if __name__ == '__main__':
    rows = make_cases()
    content = '# width category density_bits heat_bits conductivity_bits expected_bits\n'
    content += ''.join(f'{width} {kind} {r:x} {c:x} {k:x} {expected:x}\n' for width, kind, r, c, k, expected in rows)
    destination = Path(__file__).resolve().parents[1] / 'tests/fixtures/thermophysical_range.txt'
    destination.parent.mkdir(exist_ok=True)
    destination.write_text(content)
    print(len(rows), 'cases;', hashlib.sha256(content.encode()).hexdigest())
