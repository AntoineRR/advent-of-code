def main():
    first = []
    second = []
    with open("data/data.txt", "r") as file:
        for line in file:
            elts = line.split("   ")
            if len(elts) == 2:
                elt1, elt2 = elts
                first.append(int(elt1))
                second.append(int(elt2))
    first.sort()
    second.sort()

    result = 0
    for i in range(len(first)):
        result += abs(first[i] - second[i])
    print(f"Part 1 result is {result}")

    second_transformed = {}
    for elt in second:
        second_transformed[elt] = second_transformed.setdefault(elt, 0) + 1
    similarity = 0
    for elt in first:
        similarity += second_transformed.get(elt, 0) * elt
    print(f"Part 2 result is {similarity}")


if __name__ == "__main__":
    main()
