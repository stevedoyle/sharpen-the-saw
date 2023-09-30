=begin
http://projecteuler.net/index.php?section=problems&id=54
=end

########################################################################

class Card
  attr_reader :value
  attr_reader :suit

  def initialize(text)
    @suit = text[1]

    @value = case text[0]
	       when 'T' then 10
	       when 'J' then 11
	       when 'Q' then 12
	       when 'K' then 13
	       when 'A' then 14
	       else text[0].to_i
	     end
  end

  def <=>(other)
    @value <=> other.value
  end

  def >(other)
    @value > other.value
  end

  def ==(other)
    @value == other.value
  end

end

########################################################################

class Hand
  attr_reader :type
  attr_reader :cards
  attr_reader :rank_value

  def initialize(cards)
    @cards = cards.map { |c| Card.new(c) }.sort
    @type = 'none'
    @rank_value = 0
    classify

    @ranks = ['none', 'high_card', 'one_pair', 'two_pairs', 'three_of_a_kind',
      'straight', 'flush', 'full_house', 'four_of_a_kind', 'straight_flush', 'royal_flush']
  end

  def better_than?(other)
    my_rank = @ranks.index(@type)
    other_rank = @ranks.index(other.type)

    if my_rank == other_rank
      if @rank_value == other.rank_value
	@cards.reverse.zip(other.cards.reverse) do |pair|
	  next if pair[0] == pair[1]
	  return pair[0] > pair[1]
	end
      else
        @rank_value.zip(other.rank_value) do |ranks|
          next if ranks[0] == ranks[1]
          return ranks[0] > ranks[1]
        end
      end
    end

    return my_rank > other_rank    
  end

  private

  def classify
    classifiers = {
      'royal_flush'	=> lambda { royal_flush? },
      'straight_flush'	=> lambda { straight_flush? },
      'four_of_a_kind'	=> lambda { four_of_a_kind? },
      'full_house'	=> lambda { full_house? },
      'flush'		=> lambda { flush? },
      'straight'	=> lambda { straight? },
      'three_of_a_kind' => lambda { three_of_a_kind? },
      'two_pairs'	=> lambda { two_pairs? },
      'one_pair'	=> lambda { one_pair? },
      'high_card'	=> lambda { high_card? },
    }
    @type = classifiers.find { |name, pred| pred.call }[0]
  end
  
  def royal_flush?
    straight_flush? and @cards.min.value == 10
  end
  
  def straight_flush?
    straight? and flush?
  end

  def four_of_a_kind?
    num_groups_of_size(4) > 0
  end

  def full_house?
    one_pair? and three_of_a_kind?
  end

  def flush?
    return false unless @cards.group_by {|c| c.suit}.length == 1
    @rank_value = [@cards.max.value]
    return true
  end

  def straight?
    1.upto(@cards.length-1) do |i|
      return false if @cards[i].value != (@cards[i-1].value + 1)
    end
    @rank_value = [@cards.max.value]
    true
  end

  def three_of_a_kind?
    num_groups_of_size(3) > 0
  end

  def two_pairs?
    num_pairs == 2
  end

  def one_pair?
    num_pairs == 1
  end

  def high_card?
    @rank_value = [@cards.max.value]
    true
  end

  def num_pairs
    num_groups_of_size(2)
  end

  def num_groups_of_size(size)
    grps = @cards.group_by {|c| c.value}.
      select {|v,g| g.length == size}
    @rank_value = grps.keys.sort.reverse
    return grps.length
  end
end

########################################################################

if $0 == __FILE__
  results = { :p1 => 0, :p2 => 0 }

  File.new('poker.txt').each do |line|
    cards = line.strip.split(' ')
    p1 = Hand.new(cards[0..4])
    p2 = Hand.new(cards[5..-1])
    if p1.better_than? p2
      results[:p1] += 1
    else
      results[:p2] += 1
    end
  end

  puts "Player 1 won #{results[:p1]} times"

else

  describe Hand do
    it 'should recognise a royal flush' do
      h = Hand.new(['JH', 'QH', 'KH', 'AH', 'TH'])
      h.type.should == 'royal_flush'      
    end

    it 'should recognise a straight flush' do
      h = Hand.new(['2S', '3S', '4S', '5S', '6S'])
      h.type.should == 'straight_flush'      
      h.rank_value.should == [6]
    end

    it 'should recognise four of a kind' do
      h = Hand.new(['4H', '4S', '4C', '4D', '6H'])
      h.type.should == 'four_of_a_kind'
      h.rank_value.should == [4]
    end

    it 'should recognise a full house' do
      h = Hand.new(['4H', '4S', '4C', '5D', '5H'])
      h.type.should == 'full_house'
      h.rank_value.should == [4]
    end

    it 'should recognise a flush' do
      h = Hand.new(['2H', '5H', '6H', '8H', 'AH'])
      h.type.should == 'flush'
      h.rank_value.should == [14]
    end

    it 'should recognise a straight' do
      h = Hand.new(['2H', '3S', '4C', '5D', '6H'])
      h.type.should == 'straight'
      h.rank_value.should == [6]
    end

    it 'should recognise three of a kind' do
      h = Hand.new(['4H', '4S', '4C', '5D', '6H'])
      h.type.should == 'three_of_a_kind'
      h.rank_value.should == [4]
    end

    it 'should recognise two pairs' do
      h = Hand.new(['2H', '3S', '3H', '5D', '5H'])
      h.type.should == 'two_pairs'
      h.rank_value.should == [5, 3]
    end
    
    it 'should recognise one pair' do
      h = Hand.new(['2H', '2S', '3H', '4H', '9H'])
      h.type.should == 'one_pair'
      h.rank_value.should == [2]
    end

    it 'should recognise a high card' do
      h = Hand.new(['2D', '3S', '4H', 'KH', '9H'])
      h.type.should == 'high_card'
      h.rank_value.should == [13]

      h = Hand.new('3H 7H 6S KC JS'.split)
      h.type.should == 'high_card'
      h.rank_value.should == [13]
    end

    it 'should say a royal flush beats a straight flush' do
      h1 = Hand.new(['JH', 'QH', 'KH', 'AH', 'TH'])
      h2 = Hand.new(['2S', '3S', '4S', '5S', '6S'])
      h1.should be_better_than(h2)
    end

    it 'should say a flush beats another flush if it has the highest card' do
      h1 = Hand.new(['2H', '3H', '4H', '5H', 'AH'])
      h2 = Hand.new(['2S', '6S', '7S', '8S', 'AS'])
      h2.should be_better_than(h1)
    end

    it 'should pick the hand with the highest rank if both hands are two pairs' do
      h1 = Hand.new(['2H', '2D', 'KD', 'KH', 'QH'])
      h2 = Hand.new(['2C', '3C', '3S', 'AC', 'AS'])
      h2.should be_better_than(h1)
    end

    it 'should pick the hand with the highest rank if both hands are two pairs and have the same higher rank' do
      h1 = Hand.new(['2H', '2D', 'AD', 'AH', 'QH'])
      h2 = Hand.new(['2S', '3C', '3S', 'AC', 'AS'])
      h2.should be_better_than(h1)
    end

    it 'should pick the hand with the highest kicker if both hands are two pairs and have the same ranks' do
      h1 = Hand.new(['2H', '2D', 'AD', 'AH', 'QH'])
      h2 = Hand.new(['2S', '2C', 'AS', 'AC', 'KS'])
      h2.should be_better_than(h1)
    end

    it 'should pick the hand with the highest non-ranked card when both have a one pair with the sam rank' do
      h1 = Hand.new(['2H', '2D', 'KD', 'QH', 'TC'])
      h2 = Hand.new(['2C', '2S', 'KS', 'QC', 'JS'])
      h2.should be_better_than(h1)
    end

    it 'should pick the hand with the highest rank if both hands are a full house' do
      h1 = Hand.new(['2H', '2D', 'KD', 'KH', 'KC'])
      h2 = Hand.new(['2C', '2S', 'QD', 'QC', 'QS'])
      h1.should be_better_than(h2)
    end

    it 'should pick the hand with the highest card' do
      h1 = Hand.new('3H 7H 6S KC JS'.split)
      h2 = Hand.new('QH TD JC 2D 8S'.split)
      h1.should be_better_than(h2)
    end

    it 'should say the winner is player 2 in test hand 1' do
      h1 = Hand.new('5H 5C 6S 7S KD'.split)
      h2 = Hand.new('2C 3S 8S 8D TD'.split)
      h2.should be_better_than(h1)
    end

    it 'should say the winner is player 1 in test hand 2' do
      h1 = Hand.new('5D 8C 9S JS AC'.split)
      h2 = Hand.new('2C 5C 7D 8S QH'.split)
      h1.should be_better_than(h2)      
    end

    it 'should say the winner is player 2 in test hand 3' do
      h1 = Hand.new('2D 9C AS AH AC'.split)
      h2 = Hand.new('3D 6D 7D TD QD'.split)
      h2.should be_better_than(h1)      
    end

    it 'should say the winner is player 1 in test hand 4' do
      h1 = Hand.new('4D 6S 9H QH QC'.split)
      h2 = Hand.new('3D 6D 7H QD QS'.split)
      h1.should be_better_than(h2)
    end

    it 'should say the winner is player 1 in test hand 5' do
      h1 = Hand.new('2H 2D 4C 4D 4S'.split)
      h2 = Hand.new('3C 3D 3S 9S 9D'.split)
      h1.should be_better_than(h2)      
    end

  end

end
