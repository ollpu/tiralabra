"""
Plot comparison of different Criterion baselines, for a benchmark that
has a parameter. Requires matplotlib and numpy.

Usage: (run in benchmark crate root)

python3 plot_parametrized_comparison.py [benchmark path] [baseline names...]

benchmark path could be e.g. "fft/copy and fft"
"""
import sys
import json
from pathlib import Path
import numpy as np
import matplotlib
from matplotlib import pyplot as plt

benchmark_name = sys.argv[1]
baselines = sys.argv[2:]

baseline_data = { name: [] for name in baselines }

p = Path("target") / "criterion" / benchmark_name

values = []

for parameter in p.iterdir():
    value = parameter.parts[-1]
    if value != "report":
        value = int(value)
        values.append(value)
        for name in baselines:
            estimates_path = parameter / name / "estimates.json"
            with open(estimates_path) as f:
                data = json.load(f)
            a, b, c = (data["mean"]["point_estimate"] / 1000,
                       data["mean"]["confidence_interval"]["lower_bound"] / 1000,
                       data["mean"]["confidence_interval"]["upper_bound"] / 1000)
            baseline_data[name].append((value, a, b, c))

values.sort()
plt.title(benchmark_name)
plt.xscale("log")
plt.xticks(values)
plt.xlabel("input")
plt.yscale("log")
plt.ylabel("time (µs)")
plt.gca().get_xaxis().set_major_formatter(matplotlib.ticker.ScalarFormatter())
comp = [[] for x in values]
for name, data in baseline_data.items():
    data.sort(key=lambda t: t[0])
    for t, arr in zip(data, comp):
        arr.append(t[1])
    points = np.array([t[1] for t in data])
    confidence = np.array([[t[2] for t in data], [t[3] for t in data]])
    confidence = np.abs(confidence - points)
    #plt.errorbar(values, points, yerr=confidence, linestyle="solid", marker="o")
    plt.plot(values, points, label=name, marker="o")
for old, new in comp:
    change = (old-new)/old * 100
    print(f"{old:.3f}; {new:.3f}; {change:.1f} %".replace(".", ","))
plt.legend()
plt.show()
