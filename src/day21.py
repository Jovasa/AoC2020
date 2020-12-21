from collections import defaultdict


def first():

    possibles = defaultdict(list)
    ing = []
    allerg = []
    all_ingredients = set()
    with open("data/day21.txt") as f:

        for i, line in enumerate(f):
            ingredients, allergens = line.strip().split(" (contains ")
            ingredients = set(ingredients.split(" "))
            allergens = allergens.strip(")").split(", ")

            all_ingredients.update(ingredients)

            ing.append(ingredients)
            allerg.append(set(allergens))

            for a in allergens:
                possibles[a].append(i)

    all_ = {x: ing[possibles[x][0]].copy() for x in possibles}
    for allergen, ing_list in possibles.items():
        for i in ing_list:
            all_[allergen] &= ing[i]

    not_found = all_ingredients.copy()

    for ingredients in all_.values():
        not_found -= ingredients

    total = 0
    for ingredients in ing:
        total += len(not_found & ingredients)

    print(total)

    result = dict()

    temp = len(possibles)
    while len(result) != temp:
        found = None
        for k, v in all_.items():
            if len(v) == 1:
                t, *_ = v
                result[k] = t
                found = (k, t)
                break

        del all_[found[0]]

        for v in all_.values():
            try:
                v.remove(found[1])
            except KeyError:
                pass

    print(",".join(result[x] for x in sorted(result.keys())))


if __name__ == '__main__':
    first()