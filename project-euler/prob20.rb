require 'utils'

puts ((1..100).mul).to_s.chars.inject(0) { |x,y| x + y.to_i }