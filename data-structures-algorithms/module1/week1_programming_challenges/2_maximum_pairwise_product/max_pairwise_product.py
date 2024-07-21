def max_pairwise_product(numbers):
    first_max = 0
    first_max_idx = 0
    second_max = 0

    for i, number in enumerate(numbers):
        if number > first_max:
            first_max = number
            first_max_idx = i
    for i, number in enumerate(numbers):
        if number > second_max and i != first_max_idx:
            second_max = number

    return first_max * second_max


if __name__ == "__main__":
    _ = int(input())
    input_numbers = list(map(int, input().split()))
    print(max_pairwise_product(input_numbers))
