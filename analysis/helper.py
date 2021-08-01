#!/usr/bin/env python3

import subprocess
import os
import time
import json
import matplotlib.pyplot as plt


MLP_METHODS = ["kmeans", "merge", "hop"]
QUERY_METHODS = ["normal", "bi", "crp", "pch", "prp", "pch-pch"]

colors = plt.cm.Set1(range(len(MLP_METHODS)))
markers = ['s', 'X', '*', '+', 'o', '^']

identifiert = dict()

for method, color in zip(MLP_METHODS, colors):
    identifiert[method] = color

for method, marker in zip(QUERY_METHODS, markers):
    identifiert[method] = marker


def ns_to_ms(value):
    return value / 1e6


def plot_get(method):
    return identifiert[method]


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


def main():
    pass


if __name__ == '__main__':
    main()
