/*
This problem was asked by Jane Street.

cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the
first and last element of that pair. For example, car(cons(3, 4))
returns 3, and cdr(cons(3, 4)) returns 4.

Given this implementation of cons:

def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair
Implement car and cdr.
*/

package dailycodingproblem

// PairOperator is a function that can operate on a pair and return a value.
type PairOperator func(interface{}, interface{}) interface{}

// PairFunc is a function that takes a PairOperator function as an argument and returns a value.
type PairFunc func(PairOperator) interface{}

func cons(a, b interface{}) PairFunc {
	return func(f PairOperator) interface{} {
		return f(a, b)
	}
}

func car(pair PairFunc) interface{} {
	return pair(func(a, b interface{}) interface{} {
		return a
	})
}

func cdr(pair PairFunc) interface{} {
	return pair(func(a, b interface{}) interface{} {
		return b
	})
}
