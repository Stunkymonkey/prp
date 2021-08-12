#!/usr/bin/env python3

import argparse
import csv
import json
import numpy as np
import itertools
from math import radians, cos, sin, asin, sqrt


parser = argparse.ArgumentParser(description="generate eval-json")
parser.add_argument("-i", "--input", type=str, help="input fmi-file", required=True)
parser.add_argument("-o", "--output", type=str, help="output json-file", required=True)
parser.add_argument("-st", "--start-lat", type=float, help="start latitude", required=True)
parser.add_argument("-sn", "--start-lng", type=float, help="start longitude", required=True)
parser.add_argument("-et", "--end-lat", type=float, help="end latitude", required=True)
parser.add_argument("-en", "--end-lng", type=float, help="end longitude", required=True)
parser.add_argument("-w", "--walk", type=float, help="step size", required=True)
args = parser.parse_args()


def haversine(lon1, lat1, lon2, lat2):
    """
    Calculate the great circle distance between two points
    on the earth (specified in decimal degrees)
    """
    # convert decimal degrees to radians
    lon1, lat1, lon2, lat2 = map(radians, [lon1, lat1, lon2, lat2])

    # haversine formula
    dlon = lon2 - lon1
    dlat = lat2 - lat1
    a = sin(dlat / 2)**2 + cos(lat1) * cos(lat2) * sin(dlon / 2)**2
    c = 2 * asin(sqrt(a))
    r = 6371  # Radius of earth in kilometers. Use 3956 for miles
    return c * r


def convert(o):
    if isinstance(o, np.int64):
        return int(o)
        raise TypeError
    elif isinstance(o, np.ndarray):
        return o.tolist()
        raise TypeError


def main(input, output, start_lat, start_lon, end_lat, end_lon, walk):
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
    # find closes points
    start, end = 0, 0
    start_dist, end_dist = 999_999, 999_999
    for index, node in enumerate(nodes):
        s_dist = haversine(start_lon, start_lat, node["longitude"], node["latitude"])
        if s_dist < start_dist:
            start = index
            start_dist = s_dist
        e_dist = haversine(end_lon, end_lat, node["longitude"], node["latitude"])
        if e_dist < end_dist:
            end = index
            end_dist = e_dist
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
    main(args.input, args.output, args.start_lat, args.start_lng, args.end_lat, args.end_lng, args.walk)
