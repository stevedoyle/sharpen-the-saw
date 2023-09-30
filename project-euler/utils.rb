module Summable
  def sum
    inject(0) {|x,y| x+y}
  end
end

module Multiplyable
  def mul
    inject(1) {|x,y| x*y}
  end
end

class Array
  include Summable
  include Multiplyable
end

class Range
  include Summable
  include Multiplyable
end
