#!/usr/bin/env python3

import argparse
import csv
import numpy as np

parser = argparse.ArgumentParser(description="generate eval-json")
parser.add_argument("-f", "--fmi-input", type=str, help="input fmi-file", required=True)
parser.add_argument("-m", "--mlp-input", type=str, help="output mlp-file", required=True)
parser.add_argument("-o", "--output", type=str, help="output csv-file", required=True)
args = parser.parse_args()


def get_partition_id_on_level(node_id, level_id, partitions, mlp_levels):
    divisor = np.prod(mlp_levels[:level_id])
    if int(divisor) == 0:
        return partitions[node_id]
    else:
        return int(partitions[node_id] / divisor)


def get_highest_differing_level(
    node_id_a,
    node_id_b,
    partitions,
    mlp_levels,
):
    for level in range(len(mlp_levels) + 1):
        if get_partition_id_on_level(node_id_a, level, partitions, mlp_levels) == \
           get_partition_id_on_level(node_id_b, level, partitions, mlp_levels):
            return level
    raise Exception("no common level found")


def main():
    result = list()
    mlp_levels = list()
    neighbors = list()
    # read files
    with open(args.fmi_input) as csvfile:
        fmireader = csv.reader(csvfile, delimiter=' ')
        tmp = next(fmireader)
        while len(tmp) == 0 or tmp[0].startswith("#"):
            tmp = next(fmireader)
        # amount_dims = int(tmp[0])
        amount_nodes_fmi = int(next(fmireader)[0])
        amount_edges_fmi = int(next(fmireader)[0])
        for i in range(amount_nodes_fmi):
            tmp = next(fmireader)
            result.append({"id": i, "lat": tmp[2], "lon": tmp[3]})
            neighbors.append(set())
        # add edges bidirectional
        for i in range(amount_edges_fmi):
            tmp = next(fmireader)
            neighbors[int(tmp[0])].add(int(tmp[1]))
            neighbors[int(tmp[1])].add(int(tmp[0]))
    with open(args.mlp_input) as csvfile:
        mlpreader = csv.reader(csvfile, delimiter=' ')
        amount_levels = int(next(mlpreader)[0])
        for i in range(amount_levels):
            mlp_levels.append(int(next(mlpreader)[0]))
        amount_nodes_mlp = int(next(mlpreader)[0])
        assert amount_nodes_fmi == amount_nodes_mlp
        for i in range(amount_nodes_mlp):
            tmp = next(mlpreader)
            result[i].update({"partition": tmp[0]})
    # write file
    partitions = [int(row["partition"]) for row in result]
    with open(args.output, mode='w') as qgis_csv:
        qgis_writer = csv.writer(qgis_csv, delimiter=',', quotechar='"', quoting=csv.QUOTE_MINIMAL)
        header = ["lat", "lon"]
        for mlp_level_index in range(len(mlp_levels)):
            header.append("partition" + str(mlp_level_index))
        header.append("highest_diff")
        qgis_writer.writerow(header)
        for node_id, entry in enumerate(result):
            line = [entry["lat"], entry["lon"]]
            for mlp_level_index in range(len(mlp_levels)):
                line.append(get_partition_id_on_level(node_id, mlp_level_index, partitions, mlp_levels))
            highest_level = 0
            for neighbor in neighbors[node_id]:
                new_value = get_highest_differing_level(node_id, neighbor, partitions, mlp_levels)
                if new_value > highest_level:
                    highest_level = new_value
            line.append(highest_level)
            qgis_writer.writerow(line)


if __name__ == '__main__':
    main()
