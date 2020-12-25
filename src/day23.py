class Cups:

    def __init__(self, d):
        self.cups = list(int(x) for x in d)
        # self.cups.extend(range(10, 1000001))
        self.current = self.cups[0]
        self._destination = self.cups[0]
        self.picked_up = []

    @property
    def destination(self):
        return self._destination

    @destination.setter
    def destination(self, value):
        assert 1 <= value <= len(self.cups)
        self._destination = value

    def move(self):
        pick_up_index = self.cups.index(self.current) + 1
        destination_index = self.cups.index(self._destination)
        if pick_up_index < destination_index:
            if pick_up_index + 3 >= len(self.cups):
                temp = self.picked_up[len(self.cups) - pick_up_index:]
            temp = self.cups[0:pick_up_index]
            temp.extend(
                self.cups[pick_up_index + 4:destination_index + 1]
            )
            temp.extend(
                self.picked_up
            )
            temp.extend(
                self.cups[destination_index + 1:]
            )
            temp.extend(self.picked_up)
        else:
            temp = self.cups[0:destination_index + 1]
            temp.extend(
                self.cups[destination_index + 1: pick_up_index]
            )
            temp.extend(
                self.cups[pick_up_index + 4:]
            )
        print(temp)
        self.cups = temp
        self.current = self.cups[(self.cups.index(self.current) + 1) % 9]

    def pick_up(self):
        cups_index = self.cups.index(self.current)
        self.picked_up = []
        for i in range(cups_index + 1, cups_index + 4):
            self.picked_up.append(self.cups[i % len(self.cups)])

        new_dest = self.current - 1 if self.current != 1 else 9

        while new_dest in self.picked_up or new_dest == self.current:
            new_dest = new_dest - 1
            if new_dest == 0:
                new_dest = len(self.cups)

        self.destination = new_dest

    @property
    def answer(self):
        k = ""
        start = self.cups.index(1)
        for i in range(1, 2):
            k += str(self.cups[(i + start) % len(self.cups)])
        return k


def first():
    cups = Cups("193467258")
    for i in range(100):
        if (i + 1) % 10000 == 0:
            print("*", end="")
        cups.pick_up()
        cups.move()
    print(cups.answer)


if __name__ == '__main__':
    first()
