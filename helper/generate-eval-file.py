#!/usr/bin/env python3

import argparse
import csv
import json
import numpy as np


parser = argparse.ArgumentParser(description="generate eval-json")
parser.add_argument("-i", "--input", type=str, help="input fmi-file", required=True)
parser.add_argument("-o", "--output", type=str, help="output json-file", required=True)
parser.add_argument("-c", "--count", type=int, help="amount of paths", required=True)
args = parser.parse_args()


def convert(o):
    if isinstance(o, np.int64):
        return int(o)
        raise TypeError
    elif isinstance(o, np.ndarray):
        return o.tolist()
        raise TypeError


def main():
    nodes = list()
    # read files
    with open(args.input) as csvfile:
        fmireader = csv.reader(csvfile, delimiter=' ')
        tmp = next(fmireader)
        while len(tmp) == 0 or tmp[0].startswith("#"):
            tmp = next(fmireader)
        amount_dims = int(tmp[0])
        amount_nodes_fmi = int(next(fmireader)[0])
        next(fmireader)  # amount_edges
        for i in range(amount_nodes_fmi):
            tmp = next(fmireader)
            nodes.append([tmp[2], tmp[3]])
    # generate random
    rand_nodes = np.random.randint(amount_nodes_fmi, size=(args.count, 2))
    rand_pos = list()
    rand_alphas = list()
    for points in rand_nodes:
        rand_pos.append([nodes[points[0]], nodes[points[1]]])
        rand_alphas.append(np.random.dirichlet(np.ones(amount_dims), size=1))
    # export data to json
    data = dict()
    for index, (node, pos, alpha) in enumerate(zip(rand_nodes, rand_pos, rand_alphas)):
        data[index] = {"orig_start_id": node[0], "orig_end_id": node[1],
                       "start_pos": pos[0], "end_pos": pos[1], "alpha": alpha[0]}

    with open(args.output, 'w') as outfile:
        json.dump(data, outfile, ensure_ascii=False, indent=4, default=convert)


if __name__ == '__main__':
    main()
