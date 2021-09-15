#!/usr/bin/env python3

import subprocess
import os
import time
import json
import matplotlib.pyplot as plt


MLP_METHODS = ["kmeans", "merge", "gonzalez"]
QUERY_METHODS = ["normal", "bi", "pcrp", "pch", "prp", "pch-pch"]

colors = plt.cm.Set1(range(len(MLP_METHODS)))
markers = ['s', 'X', '*', '+', 'o', '^']

identifiert = dict()
mapping_colors = dict()

for method, color in zip(MLP_METHODS, colors):
    identifiert[method] = color

for method, marker in zip(QUERY_METHODS, markers):
    identifiert[method] = marker

mapping_colors["pch"] = "kmeans"
mapping_colors["pcrp"] = "gonzalez"
mapping_colors["prp"] = "merge"

TEXT_WIDTH = 426.0


def ns_to_ms(value):
    return value / 1e6


def sec_to_min(value):
    return value / 60


def plot_get(method):
    return identifiert[method]


def plot_color_get(method):
    return identifiert[mapping_colors[method]]


def mlp_title(method):
    if method == "kmeans":
        return "K-means"
    return method.title()


def shell_execute(command, EVAL_DIR):
    start_time = time.time()
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    duration = time.time() - start_time
    if result.returncode != 0:
        print("Error:", " ".join(command), "\n", result.stderr)
        raise SystemExit("Stop right there!")
    else:
        new_command = {"command": command, "stderr": result.stderr,
                       "stdout": result.stdout, "time": "{:.3f} seconds".format(duration)}
        data = dict()
        if os.path.isfile(EVAL_DIR + "/log.json"):
            with open(EVAL_DIR + "/log.json", "r") as file:
                data = json.load(file)
        data.update({time.ctime(): new_command})
        with open(EVAL_DIR + "/log.json", 'w+') as file:
            json.dump(data, file, indent=4, sort_keys=True)


def find_files_ending(ending_with, EVAL_DIR):
    files = list()
    for file in os.listdir(EVAL_DIR):
        if file.endswith(ending_with):
            tmp = os.path.join(EVAL_DIR, file)
            files.append(tmp)
    if not files:
        raise SystemExit("no input-files found!")
    return files


def out_files(input_files, input_ending, output_ending):
    output_files = list()
    for file in input_files:
        output_files.append(file.replace(input_ending, output_ending))
    return output_files


def not_created_yet(file, EVAL_DIR):
    if not os.path.isfile(os.path.join(EVAL_DIR, file)):
        print("generating:", file)
        return True
    else:
        return False


def set_pgf_size(width_pt, fraction=1, subplots=(1, 1)):
    """Set figure dimensions to sit nicely in our document.

    Parameters
    ----------
    width_pt: float
            Document width in points
    fraction: float, optional
            Fraction of the width which you wish the figure to occupy
    subplots: array-like, optional
            The number of rows and columns of subplots.
    Returns
    -------
    fig_dim: tuple
            Dimensions of figure in inches
    """
    # Width of figure (in pts)
    fig_width_pt = width_pt * fraction
    # Convert from pt to inches
    inches_per_pt = 1 / 72.27

    # Golden ratio to set aesthetic figure height
    golden_ratio = (5**.5 - 1) / 2

    # Figure width in inches
    fig_width_in = fig_width_pt * inches_per_pt
    # Figure height in inches
    fig_height_in = fig_width_in * golden_ratio * (subplots[0] / subplots[1])

    return (fig_width_in, fig_height_in)


def main():
    pass


if __name__ == '__main__':
    main()
