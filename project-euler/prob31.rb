pence = 200
$coins = [200,100,50,20,10,5,2,1]

def all_combs(sum,last) 
  return 1 if sum == 0
  return $coins.select{|d| d <= sum && d <= last}.inject(0) {|total,denom|
           total+all_combs(sum-denom,denom)}
end

puts all_combs(200, 200)