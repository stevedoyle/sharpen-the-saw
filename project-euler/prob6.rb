module Summable
  def sum
    inject(0) {|x,y| x+y}
  end
end

class Array
  include Summable
end

class Range
  include Summable
end

def sum_of_squares(limit)
  (1..limit).map {|x| x**2 }.sum
end

def square_of_sum(limit)
  (1..limit).sum**2
end

puts "Difference: #{square_of_sum(100) - sum_of_squares(100)}"
