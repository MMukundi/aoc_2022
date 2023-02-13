use std::str::FromStr;

use crate::{solution::AOCSolution, matcher::{MatchNestedList, FromStrMatcher, ShortMatchNestedListErr}};

type Value = u16;

// pub enum PacketParseErr {
//     EmptyString,
//     ExpectedOpenBracket(char)
// }

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum PacketListItem {
    Value(Value),
    List(Vec<PacketListItem>)
}
impl PartialOrd for PacketListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PacketListItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self,other) {
            (Self::Value(a),Self::Value(b))=>a.cmp(b),

            (Self::Value(_),Self::List(list))=>{
                let first = list.get(0);
                if let Some(first) = first {
                    self.cmp(first).then(if list.len()>2 {
                        std::cmp::Ordering::Less
                    }else{
                        std::cmp::Ordering::Equal
                    })
                }else{
                    std::cmp::Ordering::Greater
                }
            },
            (Self::List(list),Self::Value(_))=>{
                let first = list.get(0);
                if let Some(first) = first {
                    first.cmp(other).then(if list.len()>2 {
                        std::cmp::Ordering::Greater
                    }else{
                        std::cmp::Ordering::Equal
                    })
                }else{
                    std::cmp::Ordering::Less
                }
            },

            (Self::List(a),Self::List(b))=>a.cmp(b),

        }
    }
}
#[derive(Default,Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct PacketList(Vec<PacketListItem>);

impl Extend<Value> for PacketList {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(PacketListItem::Value))
    }
}
impl Extend<PacketList> for PacketList {
    fn extend<T: IntoIterator<Item = PacketList>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(|l|PacketListItem::List(l.0)))
    }
}

#[derive(Debug,Clone)]
pub struct PacketPairs(Vec<(PacketList,PacketList)>);
impl FromStr for PacketPairs {
    type Err= ShortMatchNestedListErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::matcher::Matcher;
        let pairs = s.split("\n\n").flat_map(|l|l.split_once("\n")).map(|(left,right)| {
            let mut matcher = MatchNestedList::<PacketList, FromStrMatcher<u16>, char, char, char>::new('[', ']', ',', FromStrMatcher::<Value>::MATCHER);
            // matcher.next_match(left).and_then(|l|matcher.next_match(right).map(|r|(l.matched,r.matched)))
            //     .expect(left)
        (
            matcher.next_match(left).expect(left).matched,
            matcher.next_match(right).expect(right).matched
        )

            // .map_err(|e|e.map(ShortMatchNestedListErr::from))
            // }).collect::<Result<Vec<_>,_>>()?;
            }).collect::<Vec<_>>();
    Ok(Self(pairs))
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=PacketPairs;
    type Part1=usize;
    type Part2=usize;
    type Err = ();
    fn solve(PacketPairs(input):Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let bad_index_sum = input.iter()
        .enumerate().filter_map(|(i,(a,b))|{
            if a<b {
                Some(i+1)
            }else{
                None
            }
        })
        .sum();

        let distress_packets = [
            (1,PacketList(vec![PacketListItem::List(vec![PacketListItem::Value(2)])])),
            (2,PacketList(vec![PacketListItem::List(vec![PacketListItem::Value(6)])])),
        ];
        let distress_packet_indices = distress_packets.map(|(offset,p)|{
            offset+input.iter().flat_map(|(a,b)|[a,b]).filter_map(|a|{
                (a<&p).then_some(())
            }).count()
        });
        Ok((bad_index_sum,distress_packet_indices.into_iter().product()))
    }
}