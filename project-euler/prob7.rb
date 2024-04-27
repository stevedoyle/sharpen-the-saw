# =begin
# By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we
# can see that the 6th prime is 13.
# What is the 10001st prime number?
# =end

require 'prime'

primes = []
gen = Prime.each
while primes.length < 10001
  primes << gen.next
end

puts primes[-1]
