use std::{fmt::Debug, marker::PhantomData, str::FromStr, mem::MaybeUninit, convert::Infallible};
use crate::or::{self, Or};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatcherOutput<'s, T> {
    pub matched: T,
    pub remaining: &'s str,
    pub chars_consumed: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatcherError<T> {
    pub index: usize,
    pub error: T,
}
impl<T> MatcherError<T> {
    #[inline]
    pub fn new(index: usize, error: T) -> Self {
        Self { index, error }
    }
    #[inline]
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> MatcherError<U> {
        MatcherError {
            index: self.index,
            error: (f)(self.error),
        }
    }
    #[inline]
    pub fn with_offset(self, offset: usize) -> Self {
        Self {
            index: self.index + offset,
            error: self.error,
        }
    }
    // #[inline]
    // pub fn offset(&mut self, offset: usize) {
    //     self.index += offset;
    // }
}

pub trait Matcher {
    type Match<'m>;
    type Error<'e>;
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>>;
    fn or<M:Matcher>(self,right:M)->OrMatcher<Self,M> where Self:Sized{
        OrMatcher::new(self, right)
    }
    fn and<M:Matcher>(self,right:M)->AndMatcher<Self,M> where Self:Sized{
        AndMatcher::new(self, right)
    }
    fn many<C>(self)->ManyMatcher<Self,C> where Self:Sized{
        ManyMatcher::new(self)
    }
}

impl Matcher for char {
    type Match<'m> = ();
    type Error<'e> = Option<char>;
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        let mut chars = source.chars();
        let first_char = chars.next();
        if first_char == Some(*self) {
            Ok(MatcherOutput {
                matched: (),
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else {
            Err(MatcherError::new(0, first_char))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrMatcherErr {
    LengthMismatch {
        min_expected_length: usize,
        source_length: usize,
    },
    CharMismatch {
        index: usize,
        expected: char,
        actual: char,
    },
}
impl Matcher for &str {
    type Match<'o> = &'o str;
    type Error<'e> = StrMatcherErr;
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        if source.len() < self.len() {
            Err(MatcherError::new(
                source.len(),
                StrMatcherErr::LengthMismatch {
                    min_expected_length: self.len(),
                    source_length: source.len(),
                },
            ))
        } else {
            let source_chars = source.chars();
            let expected_chars = self.chars();
            let mismatched_char =
                expected_chars
                    .zip(source_chars)
                    .enumerate()
                    .find_map(|(i, (expected, source))| {
                        (expected != source).then_some((i, expected, source))
                    });
            if let Some((index, expected, actual)) = mismatched_char {
                Err(MatcherError::new(
                    index,
                    StrMatcherErr::CharMismatch {
                        index,
                        expected,
                        actual,
                    },
                ))
            } else {
                let len = self.len();
                Ok(MatcherOutput {
                    matched: &source[..len],
                    remaining: &source[len..],
                    chars_consumed: len,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchNestedListErr<'s, C: Matcher, S: Matcher, I: Matcher> {
    ItemBeforeOpen(I::Match<'s>),
    CloseWithoutMatchingOpen(C::Match<'s>),
    ExpectedItemOrClose(I::Error<'s>, C::Error<'s>),
    // ExpectedSeparatorOrClose(S::Error<'s>, C::Error<'s>),
    ExpectedSeparator(S::Error<'s>),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortMatchNestedListErr {
    ItemBeforeOpen,
    CloseWithoutMatchingOpen,
    ExpectedItemOrClose,
    // ExpectedSeparatorOrClose,
    ExpectedSeparator,
}
impl<C: Matcher, S: Matcher, I: Matcher> From<MatchNestedListErr<'_, C, S, I>>
    for ShortMatchNestedListErr
{
    fn from(value: MatchNestedListErr<'_, C, S, I>) -> Self {
        match value {
            MatchNestedListErr::ItemBeforeOpen(_) => Self::ItemBeforeOpen,
            MatchNestedListErr::CloseWithoutMatchingOpen(_) => Self::CloseWithoutMatchingOpen,
            MatchNestedListErr::ExpectedItemOrClose(_, _) => Self::ExpectedItemOrClose,
            // MatchNestedListErr::ExpectedSeparatorOrClose(_, _) => Self::ExpectedSeparatorOrClose,
            MatchNestedListErr::ExpectedSeparator(_) => Self::ExpectedSeparator,
        }
    }
}
pub struct MatchNestedList<T, I, O, C, S> {
    open: O,
    close: C,
    separator: S,
    item: I,
    phantom_collection: PhantomData<T>,
}
impl<T, I, O, C, S> MatchNestedList<T, I, O, C, S> {
    pub fn new(open: O, close: C, separator: S, item: I) -> Self {
        Self {
            open,
            close,
            separator,
            item,
            phantom_collection: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManyMatcher<M,C> {
    matcher:M,
    phantom_collection:PhantomData<C>,
}
impl<M,C> ManyMatcher<M,C> {
    pub fn new(matcher:M) -> Self {
        Self { matcher,phantom_collection:PhantomData }
    }
}
impl<M: Matcher, C: for<'m> FromIterator<M::Match<'m>>> Matcher for ManyMatcher<M,C>
    where for<'s>Result<MatcherOutput<'s, M::Match<'s>>, MatcherError<M::Error<'s>>>:Debug
{
    type Match<'m> = (C,MatcherError<M::Error<'m>>);
    type Error<'e> = Infallible;
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        let mut chars_consumed = 0;
        let mut remaining = source;
        let mut error = MaybeUninit::uninit();
        let collection = std::iter::from_fn(||{
            match self.matcher.next_match(remaining){
                Ok(matched) =>{
                    chars_consumed+=matched.chars_consumed;
                    remaining=matched.remaining;
                    Some(matched.matched)
                }
                Err(err)=>{
                    error.write(err);
                    None
                }
            }
        }).collect::<C>();
        Ok(MatcherOutput{
            matched:(collection, unsafe {error.assume_init()}),
            remaining,
            chars_consumed
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AndMatcher<A,B> {
    a: A,
    b: B,
}
impl<A, B> AndMatcher<A, B> {
    pub fn new(a:A,b:B) -> Self {
        Self { a,b }
    }
}
impl<A: Matcher, B: Matcher> Matcher for AndMatcher<A,B> {
    type Match<'m> = (A::Match<'m>, B::Match<'m>);
    type Error<'e> = Or<A::Error<'e>, B::Error<'e>>;
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        match self.a.next_match(source) {
            Ok(a_match) => match self.b.next_match(a_match.remaining) {
                Ok(b_match) => Ok(MatcherOutput {
                    matched: (a_match.matched,b_match.matched),
                    remaining: b_match.remaining,
                    chars_consumed: a_match.chars_consumed+b_match.chars_consumed,
                }),
                Err(b_err) => Err(b_err.map(Or::Right).with_offset(a_match.chars_consumed)),
            },
            Err(a_err) => Err(a_err.map(Or::Left)),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrMatcher<L, R> {
    left: L,
    right: R,
}
impl<L, R> OrMatcher<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}
impl<L: Matcher, R: Matcher> Matcher for OrMatcher<L, R> {
    type Match<'m> = Or<L::Match<'m>, R::Match<'m>>;
    type Error<'e> = (L::Error<'e>, R::Error<'e>);
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        match self.left.next_match(source) {
            Ok(l) => Ok(MatcherOutput {
                matched: Or::Left(l.matched),
                remaining: l.remaining,
                chars_consumed: l.chars_consumed,
            }),
            Err(l_err) => match self.right.next_match(source) {
                Ok(r) => Ok(MatcherOutput {
                    matched: Or::Right(r.matched),
                    remaining: r.remaining,
                    chars_consumed: r.chars_consumed,
                }),
                Err(r_err) => Err(MatcherError::new(l_err.index, (l_err.error, r_err.error))),
            },
        }
    }
}

pub struct CharBoundaries<'s> {
    source: &'s str,
    char_boundary: Option<usize>,
}
impl<'s> From<&'s str> for CharBoundaries<'s> {
    fn from(source: &'s str) -> Self {
        Self {
            source,
            char_boundary: Some(0),
        }
    }
}
impl<'s> Iterator for CharBoundaries<'s> {
    type Item = (usize, &'s str);

    fn next(&mut self) -> Option<Self::Item> {
        let char_boundary = self.char_boundary?;
        self.char_boundary = (char_boundary + 1..=self.source.len())
            .find(|&index| self.source.is_char_boundary(index));
        Some((char_boundary, &self.source[..char_boundary]))
    }
}

#[derive(Debug, Default,Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStrMatcher<T>(PhantomData<T>);
impl<M> FromStrMatcher<M> {
    pub const MATCHER: Self = Self(PhantomData);
}

impl<M: FromStr> Matcher for FromStrMatcher<M> {
    type Match<'s> = M;
    type Error<'s> = M::Err;
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        let mut boundaries = CharBoundaries::from(source).enumerate();
        while let Some((char_index, (byte_index, current_str))) = boundaries.next() {
            if let Ok(matched) = current_str.parse::<M>().map(|matched| MatcherOutput {
                chars_consumed: char_index,
                matched,
                remaining: &source[byte_index..],
            }) {
                let mut to_return = matched;
                while let Some((char_index, (byte_index, current_str))) = boundaries.next() {
                    if let Ok(matched) = current_str.parse::<M>().map(|matched| MatcherOutput {
                        chars_consumed: char_index,
                        matched,
                        remaining: &source[byte_index..],
                    }) {
                        to_return = matched
                    }else{
                        break
                    }
                }
                return Ok(to_return);
            }
        }
        source
            .parse::<M>()
            .map(|matched| MatcherOutput {
                chars_consumed: source.len(),
                matched,
                remaining: "",
            })
            .map_err(|e| MatcherError::new(0, e))
    }
}
impl<M: Matcher> Matcher for &mut M {
    type Error<'e> = M::Error<'e>;
    type Match<'m> = M::Match<'m>;
    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        M::next_match(self, source)
    }
}

impl<
        O: Matcher,
        C: Matcher,
        S: Matcher,
        I: Matcher,
        T: Default + for<'s> Extend<I::Match<'s>> + Extend<T>,
    > Matcher for MatchNestedList<T, I, O, C, S>
where
    for<'s> O::Match<'s>: Debug,
    for<'s> I::Match<'s>: Debug,
{
    type Match<'s> = T;
    type Error<'e> = MatchNestedListErr<'e, C, S, I>;
    fn next_match<'s>(
        &mut self,
        string: &'s str,
    ) -> Result<MatcherOutput<'s, Self::Match<'s>>, MatcherError<Self::Error<'s>>> {
        let mut remaining_string = string;
        let mut total_chars_consumed = 0;
        let mut collections = Vec::<T>::default();

        loop {
            // Parse a bunch of opens
            while let Ok(MatcherOutput {
                remaining,
                chars_consumed,
                ..
            }) = self.open.next_match(remaining_string)
            {
                remaining_string = remaining;
                total_chars_consumed += chars_consumed;
                collections.push(T::default())
            }
            // Parse item
            let mut item_or_close = OrMatcher::new(&mut self.item, &mut self.close);
            let MatcherOutput {
                matched: item_or_close,
                remaining,
                chars_consumed,
            } = item_or_close.next_match(remaining_string).map_err(|e| {
                e.map(|(i, c)| MatchNestedListErr::ExpectedItemOrClose(i, c))
                    .with_offset(total_chars_consumed)
            })?;
            remaining_string = remaining;
            total_chars_consumed += chars_consumed;
            if let Or::Left(item) = item_or_close {
                if let Some(last_mut) = collections.last_mut() {
                    last_mut.extend([item]);
                } else {
                    break Err(MatcherError::new(
                        total_chars_consumed,
                        MatchNestedListErr::ItemBeforeOpen(item),
                    ));
                }
            } else if let Or::Right(close_match) = item_or_close {
                let Some(closed) = collections.pop() else {
                    return Err(MatcherError::new(total_chars_consumed,MatchNestedListErr::CloseWithoutMatchingOpen(close_match)))
                };
                if let Some(last_mut) = collections.last_mut() {
                    last_mut.extend([closed]);
                } else {
                    return Ok(MatcherOutput {
                        matched: closed,
                        remaining: remaining_string,
                        chars_consumed: total_chars_consumed,
                    });
                }
            }
            while let Ok(MatcherOutput {
                matched: close_match,
                remaining,
                chars_consumed,
            }) = self.close.next_match(remaining_string)
            {
                remaining_string = remaining;
                total_chars_consumed += chars_consumed;
                let Some(closed) = collections.pop() else {
                    return Err(MatcherError::new(total_chars_consumed,MatchNestedListErr::CloseWithoutMatchingOpen(close_match)))
                };
                if let Some(last_mut) = collections.last_mut() {
                    last_mut.extend([closed]);
                } else {
                    return Ok(MatcherOutput {
                        matched: closed,
                        remaining: remaining_string,
                        chars_consumed: total_chars_consumed,
                    });
                }
            }
            let MatcherOutput {
                remaining,
                chars_consumed,
                ..
            } = self.separator.next_match(remaining_string).map_err(|e| {
                e.map(|s| MatchNestedListErr::ExpectedSeparator(s))
                    .with_offset(total_chars_consumed)
            })?;
            remaining_string = remaining;
            total_chars_consumed += chars_consumed;
        }
    }
}

#[derive(Debug,Clone)]
pub struct Delimeted<C,I,D>{
    item:I,
    delimeter:D,
    phantom_collection:PhantomData<C>
}
impl<I, D, C> Delimeted<C,I,D> {
    pub fn new(item:I,delimeter:D) -> Self {
        Self {
            delimeter,
            item,
            phantom_collection: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum DelimetedError<'e,I:Matcher,D:Matcher>{
    ExpectedItem(I::Error<'e>),
    ExpectedDelimeter(D::Error<'e>),
}
impl <I:Matcher+Debug,D:Matcher+Debug,C:Default+for<'a> Extend<I::Match<'a>>> Matcher for Delimeted<C,I,D>
    where for<'e> I::Error<'e>:Debug,
    for<'e>D::Error<'e>:Debug,
    for<'e> I::Match<'e>:Debug,
    for<'e>D::Match<'e>:Debug,
    C:Debug
{
    type Match<'m>= C;

    type Error<'e>= DelimetedError<'e,I,D>;

    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<crate::matcher::MatcherOutput<'s, Self::Match<'s>>, crate::matcher::MatcherError<Self::Error<'s>>> {
        let mut collection = C::default();
        let first_item = self.item.next_match(source).map_err(|e|e.map(DelimetedError::ExpectedItem))?;
        collection.extend([first_item.matched]);
        let mut remaining = first_item.remaining;
        let mut chars_consumed = first_item.chars_consumed;

        loop {
            // println!("Test {:?}",collection);
            let Ok(next_delim) = self.delimeter.next_match(remaining).map_err(|e|e.map(DelimetedError::<I,D>::ExpectedDelimeter).with_offset(chars_consumed)) else {
                break;
            };
            chars_consumed+=next_delim.chars_consumed;
            remaining = next_delim.remaining;

            let next_item = (self.item.next_match(remaining).map_err(|e|e.map(DelimetedError::<I,D>::ExpectedItem).with_offset(chars_consumed)))?;
            collection.extend([next_item.matched]);
            chars_consumed+=next_item.chars_consumed;
            remaining = next_item.remaining;
        };
        Ok(MatcherOutput {
            matched: collection,
            remaining,
            chars_consumed,
        })
    }
}

#[derive(Debug,Clone)]
pub struct DelimetedArray<const N:usize,I,D>{
    item:I,
    delimeter:D,
}
impl<const N:usize,I, D> DelimetedArray<N,I,D> {
    pub fn new(item:I,delimeter:D) -> Self {
        Self {
            delimeter,
            item,
        }
    }
}

#[derive(Debug)]
pub enum DelimetedArrayError<'e,const N:usize,I:Matcher,D:Matcher>{
    ExpectedItem {
        index:usize,
        error:I::Error<'e>
    },
    ExpectedDelimeter( D::Error<'e>),
}
impl <const N:usize,I:Matcher+Debug,D:Matcher+Debug> Matcher for DelimetedArray<N,I,D>
    where for<'e> I::Error<'e>:Debug,
    for<'e>D::Error<'e>:Debug,
    for<'e> I::Match<'e>:Debug,
    for<'e>D::Match<'e>:Debug
{
    type Match<'m>= [I::Match<'m>;N];

    type Error<'e>= DelimetedArrayError<'e,N,I,D>;

    fn next_match<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<crate::matcher::MatcherOutput<'s, Self::Match<'s>>, crate::matcher::MatcherError<Self::Error<'s>>> {
        let mut remaining = source;
        let mut chars_consumed = 0;
        let mut items = [();N].map(|_|MaybeUninit::uninit());
        if N != 0 {

            let first_item = self.item.next_match(source).map_err(|e|e.map(|error|{
                DelimetedArrayError::ExpectedItem {
                    error,
                    index:0,
                }
            }))?;
            let mut item_iter = items.iter_mut().enumerate();
            if let Some((_,first_slot)) = item_iter.next() {
                first_slot.write(first_item.matched);
            }

            remaining = first_item.remaining;
            chars_consumed = first_item.chars_consumed;
            for (index,item) in item_iter {
                let Ok(next_delim) = self.delimeter.next_match(remaining).map_err(|e|e.map(DelimetedArrayError::<N,I,D>::ExpectedDelimeter).with_offset(chars_consumed)) else {
                    break;
                };
                
                chars_consumed+=next_delim.chars_consumed;
                remaining = next_delim.remaining;

                let next_item = (self.item.next_match(remaining).map_err(|e|e.map(|error|{
                    DelimetedArrayError::ExpectedItem {
                        error,
                        index,
                    }
                }).with_offset(chars_consumed)))?;
                item.write(next_item.matched);
                chars_consumed+=next_item.chars_consumed;
                remaining = next_item.remaining;
            };
        }
        Ok(MatcherOutput {
            matched: items.map(|item|unsafe{item.assume_init()}),
            remaining,
            chars_consumed,
        })
    }
}