use crate::solvers::{Solution, Solver};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<u8> for Card {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'2' => Ok(Self::Two),
            b'3' => Ok(Self::Three),
            b'4' => Ok(Self::Four),
            b'5' => Ok(Self::Five),
            b'6' => Ok(Self::Six),
            b'7' => Ok(Self::Seven),
            b'8' => Ok(Self::Eight),
            b'9' => Ok(Self::Nine),
            b'T' => Ok(Self::Ten),
            b'J' => Ok(Self::Jack),
            b'Q' => Ok(Self::Queen),
            b'K' => Ok(Self::King),
            b'A' => Ok(Self::Ace),
            _ => anyhow::bail!("Invalid card value"),
        }
    }
}

impl Card {
    fn replace_jack_with_joker(self) -> Self {
        match self {
            Self::Jack => Self::Joker,
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut buckets = BTreeMap::new();
        for card in value.0.iter() {
            *buckets.entry(card).or_insert(0) += 1;
        }

        let joker_count = buckets.remove(&Card::Joker).unwrap_or_default();

        if joker_count == 5 {
            return Self::FiveOfAKind;
        }

        let mut card_counts: Vec<_> = buckets.values().copied().collect();
        card_counts.sort();
        (*card_counts.last_mut().unwrap()) += joker_count;
        match card_counts.as_slice() {
            [1, 1, 1, 1, 1] => Self::HighCard,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 3] => Self::ThreeOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 4] => Self::FourOfAKind,
            [5] => Self::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hand([Card; 5]);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_type = HandType::from(self);
        let other_type = HandType::from(other);
        if self_type != other_type {
            return Some(self_type.cmp(&other_type));
        }
        for (my, theirs) in self.0.iter().zip(other.0.iter()) {
            if my != theirs {
                return Some(my.cmp(theirs));
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct SolverImpl {
    hands: Vec<(Hand, u64)>,
}

impl<'input> Solver<'input> for SolverImpl {
    fn new(input: &'input str) -> anyhow::Result<Self> {
        let hands = input
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_at(5);
                let cards: [Card; 5] = cards
                    .bytes()
                    .map(Card::try_from)
                    .collect::<Result<Vec<_>, _>>()?
                    .try_into()
                    .map_err(|_| anyhow::Error::msg("invalid number of cards in hand"))?;
                let bid = bid.trim().parse()?;
                Ok((Hand(cards), bid))
            })
            .collect::<anyhow::Result<Vec<(Hand, u64)>>>()?;
        Ok(Self { hands })
    }

    fn solve_part_1(&self) -> anyhow::Result<Solution> {
        let mut hands = self.hands.clone();
        hands.sort();
        let winnings: u64 = hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u64 + 1) * bid)
            .sum();
        Ok(Solution::with_description(
            "Total winnings",
            winnings.to_string(),
        ))
    }

    fn solve_part_2(&self) -> anyhow::Result<Solution> {
        let mut hands: Vec<_> = self
            .hands
            .iter()
            .map(|(hand, bid)| {
                let hand = Hand(
                    hand.0
                        .iter()
                        .map(|card| card.replace_jack_with_joker())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                );
                (hand, *bid)
            })
            .collect();
        hands.sort();
        let winnings: u64 = hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u64 + 1) * bid)
            .sum();
        Ok(Solution::with_description(
            "Total winnings with jokers",
            winnings.to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::SolverImpl;
    use crate::solvers::Solver;

    #[test]
    fn test_example_part_1() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day7-1.example"))?;
        assert_eq!(solver.solve_part_1()?.solution, "6440");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> anyhow::Result<()> {
        let solver = SolverImpl::new(include_str!("./day7-1.example"))?;
        assert_eq!(solver.solve_part_2()?.solution, "5905");
        Ok(())
    }
}
