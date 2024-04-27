# Find the sum of all the multiples of 3 or 5 below 1000

if __FILE__ == $0
  # TODO Generated stub
  puts (1...1000).select { |x| (x % 3 == 0) or (x % 5 == 0) }.inject(0) { |x, total| total + x }
end
