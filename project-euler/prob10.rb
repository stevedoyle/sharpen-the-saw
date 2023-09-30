# =begin
# Find the sum of all the primes below two million.
# =end

require 'prime'
require 'utils'

primes = []
Prime.each(2e6) { |x| primes << x}
puts primes.sum
