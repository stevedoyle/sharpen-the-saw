require 'set'

def triangle(n)
  n*(n+1)/2
end

def pentagonal(n)
  n*(3*n-1)/2
end

def hexagonal(n)
  n*(2*n-1)
end

puts triangle(285)
puts pentagonal(165)
puts hexagonal(143)

p = Set.new
h = Set.new
n = 1

while true
  t = triangle(n)
  p.add(pentagonal(n))
  h.add(hexagonal(n))

  if p.include?(t) and h.include?(t) and n > 285
    puts "t = #{t}"
    break
  end
  n += 1
end
