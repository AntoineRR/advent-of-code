from copy import deepcopy


def main():
    reports = []
    with open("data/data.txt", "r") as file:
        for line in file:
            reports.append(list(map(lambda x: int(x), line.split(" "))))
    n_safe = 0
    for report in reports:
        previous = report[0]
        increase = report[1] > report[0]
        safe = True
        for elt in report[1:]:
            if (elt > previous) != increase:
                safe = False
                break
            diff = abs(elt - previous)
            if diff > 3 or diff < 1:
                safe = False
                break
            previous = elt
        if safe:
            n_safe += 1

    print(f"Part 1 result is {n_safe}")

    def test_report(report: list[int], joker_used=False) -> bool:
        previous = report[0]
        increase = report[1] > report[0]
        for i, elt in enumerate(report[1:]):
            if (elt > previous) != increase:
                if joker_used:
                    return False
                report_clone_1 = deepcopy(report)
                report_clone_1.pop(i)
                report_clone_2 = deepcopy(report)
                report_clone_2.pop(i + 1)
                if test_report(report_clone_1, True) or test_report(
                    report_clone_2, True
                ):
                    return True
                else:
                    if i == 1:
                        # Pop the first element, we might have chosen increase instead of decrease
                        report_clone_1 = deepcopy(report)
                        report_clone_1.pop(i - 1)
                        if test_report(report_clone_1, True):
                            return True
                    return False
            diff = abs(elt - previous)
            if diff > 3 or diff < 1:
                if joker_used:
                    return False
                report_clone_1 = deepcopy(report)
                report_clone_1.pop(i)
                report_clone_2 = deepcopy(report)
                report_clone_2.pop(i + 1)
                if test_report(report_clone_1, True) or test_report(
                    report_clone_2, True
                ):
                    return True
                else:
                    return False
            previous = elt
        return True

    n_safe = 0
    for report in reports:
        if test_report(report):
            n_safe += 1
    print(f"Part 2 result is {n_safe}")


if __name__ == "__main__":
    main()
