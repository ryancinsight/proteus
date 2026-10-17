"""Check the guard proof constants and public transition fixture with rationals."""
from fractions import Fraction as F
from pathlib import Path
from elastic_range_reference import exact, nearest, format_spec

for width in [32, 64]:
    precision, _, _ = format_spec(width)
    u = F(2) ** (-precision)
    gamma = 4 * u / (1 - 4 * u)
    assert u < F(1, 100)
    assert 6 * gamma < 25 * u < 32 * u - u
    assert (gamma / u + 1) / (1 - gamma) < 6

fixture = Path(__file__).resolve().parent.parent / 'tests/elastic_guard_cases.txt'
for line in fixture.read_text().splitlines():
    width, lam, mu, young, bulk = line.split()
    width = int(width)
    l, m = exact(int(lam, 16), width), exact(int(mu, 16), width)
    assert l is not None and m is not None and m > 0 and 3*l + 2*m > 0
    assert nearest(m*(3*l+2*m)/(l+m), width) == int(young, 16), line
    assert nearest(l+F(2, 3)*m, width) == int(bulk, 16), line
print(f'{len(fixture.read_text().splitlines())} guard neighbors and exact proof bounds verified')
