macro_rules! count {
    () => {
        0
    };
    ($day:ident $($rest:ident)*) => {
        count!($($rest)*) + 1
    }
}
macro_rules! with_days {
    ($($day:ident),*) => {
        $(
            pub mod $day;
        )*
        pub const SOLUTIONS:[fn(usize)->String;count!($($day)*)]=[
            $(
                crate::solution::solution_string::<$day::Solution>,
            )*
        ];
    };
}
with_days!(
    day1, day2, day3, day4, day5, 
    day6, day7, day8, day9, day10, 
    day11, day12, day13, day14, day15,
    day16, day17, day18, day19, day20,
    day21,day22,day23, day24, day25
);
