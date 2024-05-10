import random
import numpy as np

MUTATION_RATE = 0.01


def calculate_path_sum(path, dist_matrix):
    return (
        sum(dist_matrix[path[i]][path[i + 1]] for i in range(len(path) - 1))
        + dist_matrix[path[-1]][path[0]]
    )


def crossover(parent1, parent2, points):
    child = (
        parent1[:points[0]]
        + parent2[points[0] : points[1]]
        + parent1[points[1]:]
    )
    return child


def mutate(individual):
    if random.random() < MUTATION_RATE:
        idx1, idx2 = random.sample(range(len(individual)), 2)
        individual[idx1], individual[idx2] = individual[idx2], individual[idx1]
        return True
    return False


def create_population(size, num_cities):
    return [random.sample(range(num_cities), num_cities) for _ in range(size)]


def select_parents(population, fitnesses):
    return [
        population[i]
        for i in np.random.choice(
            len(population), size=2, p=fitnesses / sum(fitnesses), replace=False
        )
    ]


def print_population_info(population, fitnesses, generation):
    print(f"Generation {generation+1}:")
    print("Population:", population)
    print("Distances:", fitnesses)


def genetic_algorithm(num_cities, dist_matrix, pop_size, num_generations):
    population = create_population(pop_size, num_cities)

    for generation in range(num_generations):
        fitnesses = np.array(
            [calculate_path_sum(route, dist_matrix) for route in population]
        )
        print_population_info(population, fitnesses, generation)

        new_population = []
        parent_pairs = [
            select_parents(population, fitnesses) for _ in range(pop_size // 2)
        ]
        for pair_idx, (parent1, parent2) in enumerate(parent_pairs):
            print(f"\nPair {pair_idx + 1}: {[population.index(parent1), population.index(parent2)]}")
            points = sorted(random.sample(range(1, num_cities), 2))
            print(
                "Parent 1: "
                + " ".join(map(str, parent1[:points[0]]))
                + " | "
                + " ".join(map(str, parent1[points[0] : points[1]]))
                + " | "
                + " ".join(map(str, parent1[points[1] :]))
            )
            print(
                "Parent 2: "
                + " ".join(map(str, parent2[:points[0]]))
                + " | "
                + " ".join(map(str, parent2[points[0] : points[1]]))
                + " | "
                + " ".join(map(str, parent2[points[1] :]))
            )
            child1, child2 = crossover(parent1, parent2, points), crossover(
                parent2, parent1, points
            )
            print("Child 1: " + " ".join(map(str, child1)))
            print("Child 2: " + " ".join(map(str, child2)))
            if mutate(child1):
                print("Child 1 MUTATED: " + " ".join(map(str, child1)))
            if mutate(child2):
                print("Child 2 MUTATED: " + " ".join(map(str, child2)))
            new_population.extend([child1, child2])

        population = sorted(
            population + new_population,
            key=lambda x: calculate_path_sum(x, dist_matrix),
        )[:pop_size]
        print()
        print("Enlarged population:", population)
        fitnesses = [calculate_path_sum(route, dist_matrix) for route in population]
        print("Distances:", fitnesses)
        population = population[:pop_size]
        print()

    best_route = min(population, key=lambda x: calculate_path_sum(x, dist_matrix))
    return best_route, calculate_path_sum(best_route, dist_matrix)


if __name__ == "__main__":
    num_cities = int(input("Enter the number of cities: "))
    print("Enter the distance matrix:")
    dist_matrix = [list(map(int, input().split())) for _ in range(num_cities)]
    pop_size = int(input("Enter the population size: "))
    num_generations = int(input("Enter the number of generations: "))

    best_route, best_distance = genetic_algorithm(
        num_cities, dist_matrix, pop_size, num_generations
    )
    print(f"\nBest route: {best_route}, Distance: {best_distance}")
