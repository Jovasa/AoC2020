def first():
    black_tiles = set()
    with open("data/day24.txt") as f:
        for line in f:
            t = iter(line.strip())
            x = 0
            y = 0
            for c in t:
                if c == "e":
                    x += 2
                elif c == "w":
                    x -= 2
                elif c == "n":
                    y += 1
                    c = next(t)
                    if c == "e":
                        x += 1
                    else:
                        x -= 1
                elif c == "s":
                    y -= 1
                    c = next(t)
                    if c == "e":
                        x += 1
                    else:
                        x -= 1
            if (x, y) in black_tiles:
                black_tiles.remove((x, y))
            else:
                black_tiles.add((x, y))
    print(len(black_tiles))

    for _ in range(100):
        temp = set()
        for black in black_tiles:
            n = get_neighbours(*black)
            if len(black_tiles.intersection(n)) == 1:
                temp.add(black)
            for neighbour in n:
                if len(black_tiles.intersection(get_neighbours(*neighbour))) == 2:
                    temp.add(neighbour)
        black_tiles = temp
    print(len(black_tiles))


def get_neighbours(x, y):
    temp = {
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y - 1),
        (x - 2, y),
        (x + 2, y),
    }
    return temp


if __name__ == '__main__':
    first()
