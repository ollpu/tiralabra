"""
Deriving formulas for parabolic interpolation.
"""

from sympy import *

# Premise: We have a parabola P(x), that intersects points (0, a), (1, b) and (2, c).
# We want to find the minimum point (both x and value).
a, b, c, x = symbols("a b c x")

# P(x) = k_2 * x**2 + k_1 * x + k_0
k0, k1, k2 = symbols("k_0, k_1, k_2")
P = k2 * x**2 + k1 * x + k0

# Let's solve its coefficients.
solution = solve(
    [
        P.subs({x: 0}) - a,
        P.subs({x: 1}) - b,
        P.subs({x: 2}) - c,
    ],
    (k0, k1, k2),
)
P = simplify(P.subs(solution))

print(f"P(x) =\n{P}\n")

# Find the vertex of the parabola by taking the derivative, and solving its zeros.
dP = diff(P, x)

solution, = solve(dP, x)

# Simplify and print the solution.
output = solution
print(f"minimal x =\n{output}\n")

# Find the value of the vertex by subsituting back the solution.
value = P.subs({x: solution})

output = value
print(f"minimal value =\n{output}\n")
