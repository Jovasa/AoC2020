class Cups:

    def __init__(self, d):
        self.cups = list(int(x) for x in d)
        self.current = self.cups[0]
        self._destination = self.cups[0]
        self.picked_up = []

    @property
    def destination(self):
        return self._destination

    @destination.setter
    def destination(self, value):
        assert 1 <= value <= 9
        self._destination = value

    def move(self):
        temp = []
        for i in self.cups:
            if i not in self.picked_up:
                temp.append(i)
            if i == self.destination:
                temp.extend(self.picked_up)

        self.cups = temp
        self.current = self.cups[(self.cups.index(self.current) + 1) % 9]

    def pick_up(self):
        cups_index = self.cups.index(self.current)
        self.picked_up = []
        for i in range(cups_index + 1, cups_index + 4):
            self.picked_up.append(self.cups[i % 9])

        new_dest = self.current - 1 if self.current != 1 else 9

        while new_dest in self.picked_up or new_dest == self.current:
            new_dest = new_dest - 1
            if new_dest == 0:
                new_dest = 9

        self.destination = new_dest

    @property
    def answer(self):
        k = ""
        start = self.cups.index(1)
        for i in range(1, 9):
            k += str(self.cups[(i + start) % 9])
        return k


def first():
    cups = Cups("193467258")
    for _ in range(100):
        cups.pick_up()
        cups.move()
    print(cups.answer)


if __name__ == '__main__':
    first()
