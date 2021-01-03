import numpy as np


class Layer:

    def __init__(self, t, data=None):
        self.data = np.zeros((t, t), dtype=np.bool)
        if data is not None:
            for y, line in enumerate(data):
                for x, value in enumerate(line):
                    if value == "#":
                        self.data[y, x] = True
        self._swap = np.zeros((t, t), dtype=np.bool)

    def extend(self):
        temp = self.data.shape[0] + 2
        a = np.zeros((temp, temp), dtype=np.bool)
        a[1:-1, 1:-1] = self.data
        self.data = a
        self._swap = np.zeros((temp, temp), dtype=np.bool)

    def check(self, x, y, above_layer, below_layer):
        active = self.data[y, x]

        count = self._check_layer(x, y, self.data)
        if count < 5:
            count = self._check_layer(x, y, above_layer, count)
        if count < 5 and below_layer is not None:
            count = self._check_layer(x, y, below_layer, count)

        if active and (count == 3 or count == 4):
            self._swap[y, x] = True
        elif not active and count == 3:
            self._swap[y, x] = True

    @staticmethod
    def _check_layer(x, y, layer, count=0):
        for item in layer[y - 1: y + 2]:
            for t in item[x - 1: x + 2]:
                if t:
                    count += 1

            if count > 4:
                break
        return count

    def swap(self):
        self.data = self._swap

    def print(self):
        for y in self.data[1:-1]:
            for x in y[1:-1]:
                print("#" if x else ".", end="")
            print()

    def count(self):
        self.data.flatten()
        _, counts = np.unique(self.data, return_counts=True)
        return counts[1] if len(counts) == 2 else 0


def first():
    with open("data/day17.txt") as d:
        data = d.read().strip().split()
        temp = Layer(len(data), data)
        temp.extend()
        layers = [temp]

        for _ in range(6):
            t = layers[-1].data.shape[0]
            layers.append(Layer(t))
            for layer in layers:
                layer.extend()
            for y in range(1, t + 1):
                for x in range(1, t + 1):
                    layers[0].check(x, y, layers[1].data, layers[1].data)
                    for i in range(1, len(layers) - 1):
                        layers[i].check(x, y, layers[i + 1].data, layers[i - 1].data)
                    layers[-1].check(x, y, layers[-2].data, None)

            print("round", _)
            for layer in layers:
                layer.swap()
                # layer.print()
                # print()

        counts = [x.count() for x in layers]
        total = counts[0] + sum(2*x for x in counts[1:])
        print(total)


if __name__ == '__main__':
    first()
