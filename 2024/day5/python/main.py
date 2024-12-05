from copy import deepcopy


def check_update(rules: list[list[int]], update: list[int]) -> bool:
    for i, x in enumerate(update):
        for rule in rules:
            if rule[0] == x:
                try:
                    j = update.index(rule[1])
                    if j != -1 and j < i:
                        return False
                except Exception:
                    pass
            elif rule[1] == x:
                try:
                    j = update.index(rule[0])
                    if j != -1 and j > i:
                        return False
                except Exception:
                    pass
    return True


def fix_update(rules: list[list[int]], update: list[int]):
    correct = deepcopy(update)
    for x in update:
        for rule in rules:
            if rule[0] == x:
                try:
                    i = correct.index(rule[0])
                    j = correct.index(rule[1])
                    if j != -1 and j < i:
                        correct[i] = rule[1]
                        correct[j] = rule[0]
                        correct = fix_update(rules, correct)
                except Exception:
                    pass
            elif rule[1] == x:
                try:
                    i = correct.index(rule[1])
                    j = correct.index(rule[0])
                    if j != -1 and j > i:
                        correct[i] = rule[0]
                        correct[j] = rule[1]
                        correct = fix_update(rules, correct)
                except Exception:
                    pass
    return correct


def main():
    rules = []
    updates = []
    is_rule = True
    with open("data/data.txt", "r") as file:
        for line in file:
            if line == "\n":
                is_rule = False
                continue
            if is_rule:
                rules.append(list(map(lambda x: int(x.strip()), line.split("|"))))
            else:
                updates.append(list(map(lambda x: int(x.strip()), line.split(","))))

    result = 0
    incorrect_updates = []
    for update in updates:
        if check_update(rules, update):
            result += update[len(update) // 2]
        else:
            incorrect_updates.append(update)
    print(f"Part 1 result is {result}")

    result = 0
    for update in incorrect_updates:
        correct = fix_update(rules, update)
        result += correct[len(correct) // 2]
    print(f"Part 2 result is {result}")


if __name__ == "__main__":
    main()
