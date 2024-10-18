# Difficulty: Hard

# Given an arithmetic expression in Reverse Polish Notation, write a program
# to evaluate it.

# The expression is given as a list of numbers and operands.
# For example: [5, 3, '+'] should return 5 + 3 = 8.
#
# For example, [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-']
# should return 5, since it is equivalent to ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1)) = 5.


def rpn(expr):
    stack = []
    for item in expr:
        if item not in ["+", "-", "*", "/"]:
            stack.append(item)
            continue

        a = stack.pop()
        b = stack.pop()
        if item == "+":
            stack.append(a + b)
        elif item == "-":
            stack.append(a - b)
        elif item == "*":
            stack.append(a * b)
        elif item == "/":
            stack.append(a / b)
    return stack.pop()


class TestRpn:
    def test_example1(self):
        expr = [15, 7, 1, 1, "+", "-", "/", 3, "*", 2, 1, 1, "+", "+", "-"]
        expected = 5
        assert rpn(expr) == expected
