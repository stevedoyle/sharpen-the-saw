=begin
  http://projecteuler.net/index.php?section=problems&id=32

  We shall say that an n-digit number is pandigital if it makes use of
  all the digits 1 to n exactly once; for example, the 5-digit number,
  15234, is 1 through 5 pandigital.

  The product 7254 is unusual, as the identity, 39  186 = 7254, containing
  multiplicand, multiplier, and product is 1 through 9 pandigital.

  Find the sum of all products whose multiplicand/multiplier/product identity
  can be written as a 1 through 9 pandigital.

  HINT: Some products can be obtained in more than one way so be sure to only
  include it once in your sum.
=end

require 'utils'

def pandigital?(x)
  digits = x.to_s.chars.to_a
  (9 == digits.uniq.length) and (not digits.include? "0")
end

products = []

(2..100).each do |i|
  start = i>9 ? 123 : 1234
  (start..10000/i+1).each do |j|
    p = i*j
    products << p if pandigital?([i, j, p].join)
  end
end

puts products.uniq.sum
