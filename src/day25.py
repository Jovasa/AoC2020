

def force_key(key, subject_number=7):
    value = subject_number
    rounds = 1
    while value != key:
        rounds += 1
        value = (value * subject_number) % 20201227
    return rounds


def transform(value, start, subject_number=7):
    for i in range(value - 1):
        start = (start * subject_number) % 20201227
    return start


if __name__ == '__main__':
    print(force_key(17807724))
    print(transform(11, 5764801, 5764801))
    t = force_key(14788856)
    print(transform(t, 19316454, 19316454))
