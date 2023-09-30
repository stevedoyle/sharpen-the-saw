# =begin
# Find the pythagorean triplet where a+b+c=1000 (a^2 + b^2 = c^2)
# =end

(1..1000).each do |a|
  (1..1000).each do |b|
    c = Math.sqrt((a**2) + (b**2))
    next if c.divmod(1)[1] != 0.0
    c = c.to_i
#    puts "# #{c} --> #{a+b+c}"
    if a+b+c == 1000
      puts "#{a}, #{b}, #{c}"
      puts a*b*c
      break
    end
  end
end