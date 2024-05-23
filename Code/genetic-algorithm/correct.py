import random
from numpy.random import choice
from pprint import pprint


MUT_PROB = 0.01


def route_length(route, matrix):
    l = 0
    for i in range(len(route) - 1):
        l += matrix[route[i]][route[i + 1]]
    l += matrix[route[-1]][route[0]]
    return l


def make_child(p1, p2, splits):
    child = [None] * splits[0] + p2[splits[0] : splits[1]] + [None] * (c - splits[1])
    i = 0
    j = splits[0] + 1
    stop = False
    while not stop:
        if child[i] is not None:
            i += 1
            if i >= c:
                stop = True
            continue
        while not stop:
            if p1[j] in child:
                j += 1
                if j >= c:
                    j = 0
                if j == splits[0] + 1:
                    stop = True
                continue
            child[i] = p1[j]
            break
    return child


def mutate_child(child, prob=MUT_PROB):
    if random.random() < prob:
        splits = list(choice(range(c), size=2, replace=False))
        child[splits[0]], child[splits[1]] = child[splits[1]], child[splits[0]]
        return True
    return False


def make_children(p1, p2):
    while True:
        splits = sorted(list(choice(range(c + 1), size=2, replace=False)))
        if 2 <= splits[1] - splits[0] < c:
            break
    print(
        "Parent 1: "
        + " ".join(map(str, p1[: splits[0]]))
        + " | "
        + " ".join(map(str, p1[splits[0] : splits[1]]))
        + " | "
        + " ".join(map(str, p1[splits[1] :]))
    )
    print(
        "Parent 2: "
        + " ".join(map(str, p2[: splits[0]]))
        + " | "
        + " ".join(map(str, p2[splits[0] : splits[1]]))
        + " | "
        + " ".join(map(str, p2[splits[1] :]))
    )
    c1 = make_child(p1, p2, splits)
    c2 = make_child(p2, p1, splits)
    print("Child 1: " + " ".join(map(str, c1)))
    print("Child 2: " + " ".join(map(str, c2)))
    if mutate_child(c1):
        print("Child 1 MUTATED: " + " ".join(map(str, c1)))
    if mutate_child(c2):
        print("Child 2 MUTATED: " + " ".join(map(str, c2)))
    return c1, c2


def generation(c, matrix, p, g):
    og_cities = list(range(c))
    population = sorted(
        [random.sample(og_cities, len(og_cities)) for _ in range(p)],
        key=lambda route: route_length(route, matrix),
    )
    for i in range(g):
        print(f"GENERATION {i + 1}")
        print(f"Population:")
        pprint(population)
        lengths = [route_length(route, matrix) for route in population]
        print(f"Lengths:")
        pprint(lengths)
        probabilities = [l / sum(lengths) for l in lengths]
        pairs = [
            list(choice(range(p), size=2, p=probabilities, replace=False))
            for _ in range(p // 2)
        ]
        for j, pair in enumerate(pairs):
            print(f"\nPair {j + 1}: {pair}")
            p1 = population[pair[0]]
            p2 = population[pair[1]]
            children = make_children(p1, p2)
            population += children
        print()
        population.sort(key=lambda route: route_length(route, matrix))
        print(f"Enlarged population:")
        pprint(population)
        lengths = [route_length(route, matrix) for route in population]
        print(f"Lengths:")
        pprint(lengths)
        population = population[:p]
        print()
    return population[0], route_length(population[0], matrix)


if __name__ == "__main__":
    c = int(input("Enter number of cities: "))
    print("Enter matrix: ")
    matrix = [list(map(int, input().split())) for _ in range(c)]
    p = int(input("Enter population size: "))
    g = int(input("Enter number of generations: "))
    print()
    result, length = generation(c, matrix, p, g)
    print(f"Result after {g} generations is {result} with length {length}")

    # Show statistics
    # answers = []
    # for _ in range(1000):
    #     result, length = generation(c, matrix, p, g)
    #     answers.append(length)
    # for el in sorted(set(answers)):
    #     print(el, answers.count(el))
