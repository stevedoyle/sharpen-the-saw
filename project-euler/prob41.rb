require 'Prime'

primes = []
9.downto(1) do |n|
  ([1,2,3,4,5,6,7,8,9][0...n]).permutation { |x| primes << x.join.to_i if Prime.prime?(x.join.to_i) }
  break unless primes.empty?
end

puts primes.max