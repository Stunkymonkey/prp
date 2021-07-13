#!/usr/bin/env python3

import argparse
import csv
import json
import numpy as np
import itertools


parser = argparse.ArgumentParser(description="generate eval-json")
parser.add_argument("-i", "--input", type=str, help="input fmi-file", required=True)
parser.add_argument("-o", "--output", type=str, help="output json-file", required=True)
parser.add_argument("-s", "--start", type=int, help="starting_id", required=True)
parser.add_argument("-e", "--end", type=int, help="ending_id", required=True)
parser.add_argument("-w", "--walk", type=float, help="step size", required=True)
args = parser.parse_args()


def convert(o):
    if isinstance(o, np.int64):
        return int(o)
        raise TypeError
    elif isinstance(o, np.ndarray):
        return o.tolist()
        raise TypeError


def main(input, output, start, end, walk):
    nodes = list()
    # read files
    with open(input) as csvfile:
        fmireader = csv.reader(csvfile, delimiter=' ')
        tmp = next(fmireader)
        while len(tmp) == 0 or tmp[0].startswith("#"):
            tmp = next(fmireader)
        amount_dims = int(tmp[0])
        amount_nodes_fmi = int(next(fmireader)[0])
        next(fmireader)  # amount_edges
        for i in range(amount_nodes_fmi):
            tmp = next(fmireader)
            nodes.append({"latitude": float(tmp[2]), "longitude": float(tmp[3])})
    # generate range of the alphas
    alpha_range = np.arange(0.0, 1.0000000000001, walk)
    # generating all possible combinations
    alphas = [p for p in itertools.product(alpha_range, repeat=amount_dims)]
    # only keep the ones suming up to one
    filtered_alphas = [alpha for alpha in alphas if abs(sum(alpha) - 1.0) < 0.000001]
    # export data to json
    data = list()
    for index, alpha in enumerate(filtered_alphas):
        data.append({"id": index, "orig_start_id": start, "orig_end_id": end,
                     "start_pos": nodes[start], "end_pos": nodes[end], "alpha": alpha})

    with open(output, 'w') as outfile:
        json.dump(data, outfile, ensure_ascii=False, indent=4, default=convert)


if __name__ == '__main__':
    main(args.input, args.output, args.start, args.end, args.walk)
