d = (1..1000000).entries.map {|x| x.to_s.chars.to_a}.flatten
puts [1,10,100,1000,10000,100000,1000000].inject(1) { |prod, x| prod * d[x-1].to_i }
