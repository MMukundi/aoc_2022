use std::str::FromStr;

use crate::{solution::AOCSolution, bitset::{BitSet, DefaultedBytes}};

#[derive(Debug,Clone)]
pub enum Instruction {
    Add(i16),
    Noop
}
impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("addx") => {
                parts.next().ok_or(()).and_then(|s|s.parse::<i16>().map_err(|_|{})).map(Self::Add)
            },
            Some("noop") => Ok(Self::Noop),
            _ => Err(())
        }
    }
}

#[derive(Debug,Clone)]
pub struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines().map(Instruction::from_str).collect::<Result<_,_>>().map(Instructions)
    }
}


#[derive(Debug,Default)]
pub struct ElfCPU {
    state:ElfCPUState
}
#[derive(Debug)]
pub struct ElfCPUCycles<'a,I> {
    iter:I,
    cpu:&'a mut ElfCPU,
    to_add:Option<i16>
}
impl <I:Iterator<Item=Instruction>>Iterator for ElfCPUCycles<'_,I>{
    type Item=i16;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpu.state.set_next_pixel();
        if let Some(n) = self.to_add.take() {
            self.cpu.state.x+=n;
        }else {
            // Begin a new instruction
            // Note: If there are no more instructions, the iterator should end
            self.to_add = if let Instruction::Add(n) = self.iter.next()? {
                Some(n)
            }else{
                None
            }
        }
        Some(self.cpu.state.x)
    }
}
#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub struct ElfCPUState {
    x:i16,
    pixel_index:u8,
    pixels:BitSet<u8,DefaultedBytes<30>>
}
impl ElfCPUState {
    pub fn set_next_pixel(&mut self){
        if (self.x as i16).abs_diff((self.pixel_index%40) as i16) < 2 {
            self.pixels.insert(self.pixel_index);
        }
        self.pixel_index+=1;
    }
}
impl ElfCPU {
    pub fn run<I:IntoIterator<Item=Instruction>>(&mut self,program: I)->ElfCPUCycles<I::IntoIter>{
        ElfCPUCycles { iter: program.into_iter(), cpu: self, to_add: None }
    }
}
impl Default for ElfCPUState{
    fn default() -> Self {
        Self { x: 1,pixels:Default::default(),pixel_index:0 }
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=Instructions;
    type Part1=i16;
    type Part2=String;
    type Err = ();
    fn solve(Instructions(instructions):Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let mut cpu = ElfCPU::default();
        // dbg!(&instructions);
        let mut cycle_outputs = cpu.run(instructions);

        let twentyth = cycle_outputs.nth(18).unwrap_or_default();
        let every_40th = cycle_outputs.skip(39) // get to the 20th
            .step_by(40); // Only use every 40th
        let strength_sum  = 20*twentyth+every_40th
            .enumerate()
            .map(|(i,s)|(40*i as i16+60)*s).sum::<i16>();
        let s = (0..6).flat_map(|r|std::iter::once('\n').chain((40*r..).take(40).map(|i|cpu.state.pixels.contains(i)).map(|b|if b{'#'}else{'.'}))).collect::<String>();
        Ok((strength_sum,s))
    }
}