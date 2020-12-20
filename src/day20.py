from __future__ import annotations
from enum import Enum
from typing import Dict


class Edge(Enum):
    RIGHT = 0
    BOTTOM = 1
    LEFT = 2
    TOP = 3


class Tile:

    def __init__(self, d: str):
        index, *rest = d.split("\n")
        self.index = int(index[5:9])
        self.data = rest

        self.neighbors = {x: None for x in Edge}

        self.matched_direction: Dict[int, (Edge, bool)] = {}

    def get_edge(self, edge: Edge) -> str:
        if edge == Edge.TOP:
            return self.data[0]
        if edge == Edge.BOTTOM:
            return self.data[-1]
        if edge == Edge.LEFT:
            return "".join([x[0] for x in self.data])
        if edge == Edge.RIGHT:
            return "".join([x[-1] for x in self.data])

    def match_edges(self, other: Tile):
        for my_edge in Edge:
            if self.neighbors[my_edge] is not None:
                continue

            for other_edge in Edge:
                if other.neighbors[other_edge] is not None:
                    continue

                other_data = other.get_edge(other_edge)
                my_data = self.get_edge(my_edge)
                if my_data == other_data:
                    self.neighbors[my_edge] = other
                    self.matched_direction[other.index] = (other_edge, False)

                    other.neighbors[other_edge] = self
                    other.matched_direction[self.index] = (my_edge, False)
                    return True
                elif my_data == other_data[::-1]:
                    self.neighbors[my_edge] = other
                    self.matched_direction[other.index] = (other_edge, True)

                    other.neighbors[other_edge] = self
                    other.matched_direction[self.index] = (my_edge, True)
                    return True

        return False

    def get_image(self, other, direction, rotation, mirrored):
        if not other:
            return [list(x[1:-1]) for x in self.data[1:-1]], False, 0
        dir_, mirror = self.matched_direction[other]

        dir_ = (rotation + dir_.value) % 4
        if direction == Edge.TOP:
            dir_ += 1
        if dir_ == 0 or dir_ == 4:
            data = [list(x[1:-1]) for x in self.data[1:-1]]
        elif dir_ == 2:
            data = [list(x[-2:0:-1]) for x in self.data[-2:0:-1]]
        elif dir_ == 1:
            data = []
            len_self_data = len(self.data[0]) - 1
            for i in range(1, len_self_data):
                temp = []
                for j in range(1, len_self_data):
                    temp.append(self.data[len_self_data - j][i])
                data.append(temp)
        else:
            data = []
            len_self_data = len(self.data[0]) - 1
            for i in range(1, len_self_data):
                temp = []
                for j in range(1, len_self_data):
                    temp.append(self.data[len_self_data - j][len_self_data - i])
                data.append(temp)

        mirror = mirror ^ mirrored
        if mirror:
            data = data[::-1]

        return data, mirror, dir_

    def __repr__(self):
        return str(self.index)

    def __str__(self):
        return str(self.index)


def first():
    tiles = dict()
    with open("data/day20.txt") as d_file:
        for data in d_file.read().strip().split("\n\n"):
            temp = Tile(data)
            tiles[temp.index] = temp

    temp = [x for x in sorted(tiles.keys())]

    for i in range(len(temp)):
        for j in range(i + 1, len(temp)):
            tiles[temp[i]].match_edges(tiles[temp[j]])

    temp = 1
    for tile in tiles.values():
        if sum(1 for x in tile.neighbors.values() if x is None) == 2:
            temp *= tile.index
            print(tile.neighbors)

    print(temp)

    n = 2971
    while True:
        print(tiles[n].get_image(None, Edge.LEFT, 0, False))
        print(tiles[1489].neighbors[Edge.RIGHT], tiles[1171].matched_direction[1489])
        break


if __name__ == '__main__':
    first()
