def find_letter_nearby(
    input: list[list[str]], x: int, y: int, letter: str
) -> list[tuple[int, int]]:
    result = []
    if x > 0 and y > 0 and input[y - 1][x - 1] == letter:
        result.append((-1, -1))
    if x > 0 and input[y][x - 1] == letter:
        result.append((-1, 0))
    if x > 0 and y < len(input) - 1 and input[y + 1][x - 1] == letter:
        result.append((-1, 1))
    if y > 0 and input[y - 1][x] == letter:
        result.append((0, -1))
    if y < len(input) - 1 and input[y + 1][x] == letter:
        result.append((0, 1))
    if y > 0 and x < len(input[y - 1]) - 1 and input[y - 1][x + 1] == letter:
        result.append((1, -1))
    if x < len(input[y]) - 1 and input[y][x + 1] == letter:
        result.append((1, 0))
    if (
        y < len(input) - 1
        and x < len(input[y + 1]) - 1
        and input[y + 1][x + 1] == letter
    ):
        result.append((1, 1))
    return result


def main():
    input = []
    with open("data/data.txt", "r") as file:
        for line in file:
            input.append(line)

    count = 0
    for y, line in enumerate(input):
        for x, letter in enumerate(line):
            if letter == "X":
                dirs = find_letter_nearby(input, x, y, "M")
                if dirs:
                    for dir in dirs:
                        if (
                            y + 2 * dir[1] < 0
                            or y + 2 * dir[1] > len(input) - 1
                            or x + 2 * dir[0] < 0
                            or x + 2 * dir[0] > len(input[0]) - 1
                        ):
                            continue
                        if input[y + 2 * dir[1]][x + 2 * dir[0]] == "A":
                            if (
                                y + 3 * dir[1] < 0
                                or y + 3 * dir[1] > len(input) - 1
                                or x + 3 * dir[0] < 0
                                or x + 3 * dir[0] > len(input[0]) - 1
                            ):
                                continue
                            if input[y + 3 * dir[1]][x + 3 * dir[0]] == "S":
                                count += 1
    print(f"Part 1 result is {count}")

    count = 0
    for y, line in enumerate(input[1:-1]):
        for x, letter in enumerate(line[1:-1]):
            if letter == "A":
                true_x = x + 1
                true_y = y + 1
                if (
                    input[true_y - 1][true_x - 1] in ["M", "S"]
                    and input[true_y + 1][true_x + 1] in ["M", "S"]
                    and input[true_y + 1][true_x + 1] != input[true_y - 1][true_x - 1]
                ):
                    if (
                        input[true_y + 1][true_x - 1] in ["M", "S"]
                        and input[true_y - 1][true_x + 1] in ["M", "S"]
                        and input[true_y - 1][true_x + 1]
                        != input[true_y + 1][true_x - 1]
                    ):
                        count += 1
    print(f"Part 2 result is {count}")


if __name__ == "__main__":
    main()
