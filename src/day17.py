import numpy as np


class Layer:

    def __init__(self, t):
        self.data = np.zeros((t, t), dtype=np.bool)
        self.swap = np.zeros((t, t), dtype=np.bool)

    def extend(self):
        temp = self.data.shape[0] + 2
        a = np.zeros((temp, temp), dtype=np.bool)
        a[1:-1, 1:-1] = self.data
        self.data = a

    def check(self, x, y, above_layer, below_layer):
        active = self.data[y, x]

        count 
        for item in self.data[y - 1: y + 2]:



def first():
    pass
