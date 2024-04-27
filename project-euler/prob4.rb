class Fixnum
  def palindrome?
    x = to_s
    x == x.reverse
  end
end

palindromes = []
(100..999).each do |first|
  (100..999).each do |second|
    product = first * second
    palindromes << product if product.palindrome?
  end
end

puts palindromes
print "Largest palindrome: "
puts palindromes.max
