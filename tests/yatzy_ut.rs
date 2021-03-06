#[cfg(test)]
use demonstrate::demonstrate;

#[cfg(test)]
fn roll(a: u32, b: u32, c: u32, d: u32, e: u32) -> Vec<u32> {
    vec![a, b, c, d, e]
}

#[cfg(test)]
demonstrate! {
    // The game of yatzy is a simple dice game. Each player rolls five six-sided dice. The player places the roll in a category, such as ones, twos, fives, pair, two pairs etc (see below). If the roll is compatible with the category, the player gets a score for the roll according to the rules. If the roll is not compatible with the category, the player scores zero for the roll.
    //
    // For example, if a player rolls 5,6,5,5,2 and scores the dice in the fives category they would score 15 (three fives).
    //
    // Your task is to score a GIVEN roll in a GIVEN category.
    // You do NOT have to program the random dice rolling.
    // You do NOT have to program re-rolls (as in the real game).
    // You do NOT play by letting the computer choose the highest scoring category for a given roll.
    
    
    // Yatzy Categories and Scoring Rules
    // ==================================
    describe "In a yatzy game with five rolls" {
        use super::roll;
        use yatzy::*;
        use hamcrest2::prelude::*;

        it "chance scores the sum of all dice, no matter what they read" {
            assert_that!(yatzy(Category::Chance, roll(1,1,3,3,6)), eq(1+1+3+3+6));
            assert_that!(yatzy(Category::Chance, roll(4,5,5,6,1)), eq(4+5+5+6+1));
        }

        it "Yatzy: If all dice have the same number, the player scores 50 points." {
            assert_that!(yatzy(Category::Yatzy, roll(1,1,1,1,1)), eq(50));
            assert_that!(yatzy(Category::Yatzy, roll(5,5,5,5,5)), eq(50));
            assert_that!(yatzy(Category::Yatzy, roll(1,1,1,2,1)), eq(0));
        }

        it "Ones scores the sum of dice that read one" {
            assert_that!(yatzy(Category::Ones, roll(3,3,3,4,5)), eq(0));
            assert_that!(yatzy(Category::Ones, roll(1,1,1,1,1)), eq(5));
        }
        it "Ones, Twos, Threes, Fours, Fives, Sixes: scores the sum of dice that reads one, two, three, four, five or six, respectively" {
            assert_that!(yatzy(Category::Ones, roll(3,3,3,4,5)), eq(0));
            assert_that!(yatzy(Category::Ones, roll(1,1,1,1,1)), eq(1+1+1+1+1));
            assert_that!(yatzy(Category::Twos, roll(2,3,2,5,1)), eq(2+2));
            assert_that!(yatzy(Category::Threes, roll(2,3,2,5,1)), eq(3));
            assert_that!(yatzy(Category::Fours, roll(1,1,2,4,4)), eq(4+4));
            assert_that!(yatzy(Category::Fives, roll(5,5,5,5,5)), eq(5+5+5+5+5));
            assert_that!(yatzy(Category::Sixes, roll(2,3,2,5,1)), eq(0));
            assert_that!(yatzy(Category::Sixes, roll(4,5,5,6,1)), eq(6));
        }

        // Pair:
        // If exactly two dice have the same value then the player scores the sum of the two highest matching dice.
        // For example, when placed on "pair"
        //    3,3,3,4,4 scores 8 (4+4)
        //    1,1,6,2,6 scores 12 (6+6)
        //    3,3,3,4,1 scores 0
        //    3,3,3,3,1 scores 0
        
        // Two pairs:
        // If exactly two dice have the same value and exactly two dice have a different value then the player scores the sum of these four dice.
        // For example, when placed on "two pairs"
        //    1,1,2,3,3 scores 8 (1+1+3+3)
        //    1,1,2,3,4 scores 0
        //    1,1,2,2,2 scores 0
        
        // Three of a kind:
        // If there are exactly three dice with the same number then the player scores the sum of these dice.
        // For example, when placed on "three of a kind"
        //     3,3,3,4,5 scores 9 (3+3+3)
        //     3,3,4,5,6 scores 0
        //     3,3,3,3,1 scores 0
        
        // Four of a kind:
        // If there are exactly four dice with the same number then the player scores the sum of these dice.
        // For example, when placed on "four of a kind"
        //     2,2,2,2,5 scores 8 (2+2+2+2)
        //     2,2,2,5,5 scores 0
        //     2,2,2,2,2 scores 0

        it "Small straight: if the dice read one, two, three, four, five, the player scores 15 (the sum of all the dice)" {
            assert_that!(yatzy(Category::SmallStraight, roll(2,3,4,5,6)), eq(0));
            assert_that!(yatzy(Category::SmallStraight, roll(1,2,3,4,5)), eq(15));
        }
        it "Large straight: if the dice read two, three, four, five, six, the player scores 20 (the sum of all the dice)" {
            assert_that!(yatzy(Category::LargeStraight, roll(2,3,4,5,6)), eq(20));
            assert_that!(yatzy(Category::LargeStraight, roll(1,2,3,4,5)), eq(0));
        }

        it "Full house: If the dice are two of a kind and three of a different kind then the player scores the sum of all five dice." {
        assert_that!(yatzy(Category::FullHouse, roll(1,1,2,2,2)), eq(1+1+2+2+2));
        assert_that!(yatzy(Category::FullHouse, roll(1,2,2,2,2)), eq(0));
        assert_that!(yatzy(Category::FullHouse, roll(2,2,3,3,4)), eq(0));
        assert_that!(yatzy(Category::FullHouse, roll(4,4,4,4,4)), eq(0));
        }
    }
}
