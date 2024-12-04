import re


def main():
    input = ""
    with open("data/data.txt", "r") as file:
        for line in file:
            input += line

    result = 0
    matches = re.finditer("mul\((\d+),(\d+)\)", input)
    for match_ in matches:
        result += int(match_[1]) * int(match_[2])

    print(f"Part 1 result is {result}")

    result = 0
    matches = re.finditer("mul\((\d+),(\d+)\)", input)
    dos = re.finditer("do\(\)", input)
    donts = re.finditer("don't\(\)", input)
    combined = sorted(list(dos) + list(donts) + list(matches), key=lambda x: x.start())
    activated = True
    for match_ in combined:
        if match_[0].startswith("mul"):
            if activated:
                result += int(match_[1]) * int(match_[2])
        elif match_[0].startswith("do()"):
            activated = True
        else:
            activated = False
    print(f"Part 2 result is {result}")


if __name__ == "__main__":
    main()
