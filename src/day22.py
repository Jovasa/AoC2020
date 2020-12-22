from collections import deque


def first():
    players = get_cards()

    while all(len(x) for x in players):
        first_ = players[0].popleft()
        second_ = players[1].popleft()
        if first_ > second_:
            players[0].extend((first_, second_))
        else:
            players[1].extend((second_, first_))

    for p in players:
        if len(p):
            score = 0
            for i, card in enumerate(reversed(p)):
                score += (i + 1) * card
            print(score)


def hash_deque(a: deque):
    hash_ = 0
    for i in a:
        hash_ = (hash_ << 6) + i
    return hash_


def recursive_combat(first_: deque, second_: deque) -> bool:
    first_states = set()
    second_states = set()
    while len(first_) and len(second_):
        if hash_deque(first_) in first_states or hash_deque(second_) in second_states:
            return True

        first_states.add(hash_deque(first_))
        second_states.add(hash_deque(second_))

        first_card = first_.popleft()
        second_card = second_.popleft()

        if len(first_) >= first_card and len(second_) >= second_card:
            first_won = recursive_combat(
                deque(first_[x] for x in range(first_card)),
                deque(second_[x] for x in range(second_card))
            )
        else:
            first_won = first_card > second_card

        if first_won:
            first_.extend((first_card, second_card))
        else:
            second_.extend((second_card, first_card))

    return len(second_) == 0


def second():
    players = get_cards()

    first_won = recursive_combat(*players)

    if first_won:
        p = players[0]
    else:
        p = players[1]

    score = 0
    for i, card in enumerate(reversed(p)):
        score += (i + 1) * card
    print(score)


def get_cards():
    with open("data/day22.txt") as f:
        data = f.read()
        players = list()
        for player in data.split("\n\n"):
            players.append(deque(int(x) for x in player.strip().split("\n")[1:]))
    return players


if __name__ == '__main__':
    first()
    second()
