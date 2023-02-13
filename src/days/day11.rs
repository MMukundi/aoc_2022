use std::{str::FromStr, collections::VecDeque, num::ParseIntError, cmp::Reverse};

use crate::solution::AOCSolution;

type Worry = u128;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum Operation {
    Mult,
    Add,
    Sub,
    Div
}
impl FromStr for Operation {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+"=>Ok(Self::Add),
            "-"=>Ok(Self::Sub),
            "*"=>Ok(Self::Mult),
            "/"=>Ok(Self::Div),
            _=>Err(())
        }
    }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum OpPart {
    Old,
    Const(Worry)
}
impl FromStr for OpPart {
    type Err=ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old"=>Ok(Self::Old),
            s=>s.parse().map(Self::Const)
        }
    }
}
#[derive(Debug,Clone)]
pub struct Monkey {
    items:VecDeque<Worry>,
    operation: (OpPart,Operation,OpPart),
    divisibility_rule:Worry,
    true_throw:usize,
    false_throw:usize,
}
impl Monkey {
    pub fn handle_next<F:Fn(Worry)->Worry>(&mut self,worry_hanlder:F)->Option<(usize,Worry)>{
        let worry = self.items.pop_front()?;

        fn eval(part:&OpPart,curr: Worry)-> Worry {
            match part {
                &OpPart::Const(w)=>w,
                OpPart::Old=>curr
            }
        }
        let left = eval(&self.operation.0,worry);
        let right = eval(&self.operation.2,worry);
        let augmented_worry =match self.operation.1 {
            Operation::Mult => left*right,
            Operation::Add => left+right,
            Operation::Sub => left-right,
            Operation::Div => left/right,
        };
        let worry_after_inspection = worry_hanlder(augmented_worry);
        let dest = if worry_after_inspection % self.divisibility_rule == 0 {
            self.true_throw
        }else{
            self.false_throw
        };
        Some((dest,worry_after_inspection))
    }
}
impl FromStr for Monkey {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        let starting_items = lines.next()
            .and_then(|l|l.split_once(": "))
            .ok_or(())
            .and_then(|(_,nums)|{
                nums.split(", ").map(Worry::from_str).collect::<Result<_,_>>().map_err(|_|{})
            })?;
            let operation = lines.next()
        .and_then(|l|{
            let (_,rhs) = l.split_once("= ")?;
            let mut lines = rhs.split_whitespace();
            let left = OpPart::from_str(lines.next()?).ok()?;
            let op = Operation::from_str(lines.next()?).ok()?;
            let right = OpPart::from_str(lines.next()?).ok()?;
            Some((left,op,right))
        })
        .ok_or(())?;
        let test_line = lines.next().ok_or(())?.split_whitespace().last().ok_or(())?.parse().map_err(|_|{})?;
        let true_line = lines.next().ok_or(())?.split_whitespace().last().ok_or(())?.parse().map_err(|_|{})?;
        let false_line = lines.next().ok_or(())?.split_whitespace().last().ok_or(())?.parse().map_err(|_|{})?;

        Ok(Self {
            items:starting_items,
            operation,
            divisibility_rule:test_line,
            true_throw:true_line,
            false_throw:false_line
        })
    }
}

#[derive(Debug,Clone)]
pub struct Monkeys(Vec<Monkey>);
impl FromStr for Monkeys {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split("\n\n").map(Monkey::from_str).collect::<Result<_,_>>()?))
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=Monkeys;
    type Part1=u128;
    type Part2=u128;
    type Err = ();
    fn solve(Monkeys(mut monkeys) :Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let mut simple_monkeys = monkeys.clone();
        let mut inspection_counts = vec![0;simple_monkeys.len()];
        let mut incoming_throws = vec![VecDeque::<Worry>::new();monkeys.len()];
        for _ in 0..20 {
            for monkey_index in 0..simple_monkeys.len() {
                let count = &mut inspection_counts[monkey_index];
                let monkey = &mut simple_monkeys[monkey_index];
                monkey.items.extend(incoming_throws[monkey_index].drain(..));
                while let Some((dest,w)) = monkey.handle_next(|w|w/3) {
                    *count+=1;
                    incoming_throws[dest].push_back(w);
                }
            }
        }
        let mut sorted_for_monkey_business = inspection_counts.clone();
        sorted_for_monkey_business.sort_by_cached_key(|a|Reverse(*a));
        let simple_mb=sorted_for_monkey_business[0]*sorted_for_monkey_business[1];



        let worry_div_prod = monkeys.iter().map(|m|m.divisibility_rule).product::<Worry>();

        let mut inspection_counts = vec![0;monkeys.len()];
        let mut incoming_throws = vec![VecDeque::<Worry>::new();monkeys.len()];
        for _ in 0..10000 {
            for monkey_index in 0..monkeys.len() {
                let count = &mut inspection_counts[monkey_index];
                let monkey = &mut monkeys[monkey_index];
                monkey.items.extend(incoming_throws[monkey_index].drain(..));
                while let Some((dest,w)) = monkey.handle_next(|w|w%worry_div_prod) {
                    *count+=1;
                    incoming_throws[dest].push_back(w);
                }
            }
        }
        let mut sorted_for_monkey_business = inspection_counts.clone();
        sorted_for_monkey_business.sort_by_cached_key(|a|Reverse(*a));
        let tough_mb=sorted_for_monkey_business[0]*sorted_for_monkey_business[1];

        Ok((simple_mb,tough_mb))
    }
}