

class Bag:
    all_ = dict()
    stuff = set()

    def __init__(self, name):
        self.name = name
        self.upper = []
        self.contains = []

    def add_upper(self, other):
        if other not in self.all_:
            self.all_[other] = Bag(other)

        self.upper.append(self.all_[other])

    def add_contain(self, other, count):
        self.contains.append((self.all_[other], count))

    def get_upper(self):
        print(self.name)
        self.stuff.add(self.name)
        if not self.upper:
            return {self.name}

        temp = {self.name}
        for bag in self.upper:
            temp.update(bag.get_upper())

        return temp

    def contents(self):
        if not self.contains:
            return 0

        total_count = 0
        for bag, count in self.contains:
            total_count += count * (bag.contents() + 1)

        return total_count


def first():

    for line in open("data/day7.txt"):
        bag, rest = line.split(" contain ")
        bag = bag[:-1]

        rest = rest.strip()

        if bag not in Bag.all_:
            Bag.all_[bag] = Bag(bag)

        if rest == "no other bags.":
            continue

        for item in rest.split(", "):
            bag_ = item[2:].strip(".").rstrip("s")
            if bag_ not in Bag.all_:
                Bag.all_[bag_] = Bag(bag_)

            Bag.all_[bag_].add_upper(bag)

            Bag.all_[bag].add_contain(bag_, int(item[0]))

    print(len(Bag.all_["shiny gold bag"].get_upper()), len(Bag.stuff))
    print(Bag.all_["shiny gold bag"].contents())


if __name__ == '__main__':
    first()