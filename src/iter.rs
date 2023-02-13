// use std::{mem::MaybeUninit};

// pub struct ArrayWindows<const N:usize, I:Iterator>{
//     array:Option<[I::Item;N]>,
//     iter:I,
// }
// impl <const N:usize, I:Iterator> ArrayWindows<N,I>{
//     pub fn new(mut iter:I)->Self{
//         let mut array = [();N].map(|_|MaybeUninit::<I::Item>::uninit());

//         let mut initialized_elements = 0usize;

//         for (array_item,item) in array.iter_mut().zip(iter.by_ref()) {
//             array_item.write(item);
//             initialized_elements+=1;
//         }

//         Self {
//             array: {
//                 if initialized_elements == N {
//                     Some(array.map(|val|unsafe {val.assume_init()}))
//                 } else {
//                     None
//                 }
//             },
//             iter
//         }
//     }
//     pub fn advance(&mut self) -> Option<&mut [I::Item;N]> {
//         let next = self.iter.next()?;
//         let array = self.array.as_mut().take()?;
//         crate::array::push_to_end(array, next);
//         Some(array)
//     }
// }
// impl <const N:usize, I:Iterator> Iterator for ArrayWindows<N,I>
//     where I::Item:Clone
// {
//     type Item=[I::Item;N];

//     fn next(&mut self) -> Option<Self::Item> {
//         let next = self.iter.next()?;
//         let array = self.array.as_mut().take()?;
//         crate::array::push_to_end(array, next);
//         Some(array.clone())
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::ArrayWindows;

//     #[test]
//     pub fn a(){
//         for i in ArrayWindows::<3,_>::new(0usize..20).step_by(3) {
//             dbg!(i);
//         }
//     }
// }

use std::{mem::MaybeUninit, ops::ControlFlow};

pub trait IterTuple:Sized{
    type Items;
    fn get_nexts(&mut self)->Option<Self::Items>;
    fn iter_tuple_next(
        source_iters: &mut Self,
        active_iters: &mut Self,
        current_values: &mut Option<Self::Items>
    )->Option<Self::Items>;
}

macro_rules! impl_iter_tup {
    (($l_type:ident,$l_value:ident,$l_source_iter:ident,$l_active_iter:ident)$(,($type:ident,$value:ident,$source_iter:ident,$active_iter:ident))*) => {
        impl <$l_type:Iterator+Clone $(,$type:Iterator+Clone)*> IterTuple for ($l_type,$($type),*)
            where <$l_type as Iterator>::Item:Clone $(,<$type as Iterator>::Item:Clone)*
        {
            type Items = (<$l_type as Iterator>::Item, $(<$type as Iterator>::Item),*);
            fn get_nexts(&mut self)->Option<Self::Items>{
                let ($l_source_iter,$($source_iter),*) = self;
                Some(($l_source_iter.next()?,$($source_iter.next()?),*))
            }
            fn iter_tuple_next(
                source_iters: &mut Self,
                active_iters: &mut Self,
                current_values: &mut Option<Self::Items>
            )->Option<Self::Items> {
                let current_values_mut = current_values.as_mut()?;
                let to_return = current_values_mut.clone();
                let ($l_value,$($value),*) = current_values_mut;
                let ($l_active_iter,$($active_iter),*) = active_iters;
                let ($l_source_iter,$($source_iter),*) = source_iters;

                if let Some(new_item) = $l_active_iter.next() { // If this slot can keep going
                    *$l_value = new_item; // Update it
                    return Some(to_return) // And return the previous_state
                }else {
                    *$l_active_iter = $l_source_iter.clone().into_iter(); // Restart the slot
                    if let Some(replacement_item) = $l_active_iter.next() {
                        *$l_value = replacement_item;// And store its first item
                    } else { // If for some reason this slot now doesn't have any items
                        *current_values = None; // Take away the current value
                        return Some(to_return) // And return the last state
                    }
                }

                $(
                    if let Some(new_item) = $active_iter.next() { // If this slot can keep going
                        *$value = new_item; // Update it
                        return Some(to_return) // And return the previous_state
                    }else {
                        *$active_iter = $source_iter.clone().into_iter(); // Restart the slot
                        if let Some(replacement_item) = $active_iter.next() {
                            *$value = replacement_item;// And store its first item
                        } else { // If for some reason this slot now doesn't have any items
                            *current_values = None; // Take away the current value
                            return Some(to_return) // And return the last state
                        }
                    }
                )*
                *current_values = None; // Take away the current value
                return Some(to_return) // And return the last state
            }
        }
        impl_iter_tup!($(($type,$value,$source_iter,$active_iter)),*);
    };
    () => {
    }
}

impl_iter_tup!(
    (A,a,a_source,a_active),
    (B,b,b_source,b_active),
    (C,c,c_source,c_active),
    (D,d,d_source,d_active),
    (E,e,e_source,e_active),
    (F,f,f_source,f_active),
    (G,g,g_source,g_active),
    (H,h,h_source,h_active),
    (I,i,i_source,i_active),
    (J,j,j_source,j_active),
    (K,k,k_source,k_active),
    (L,l,l_source,l_active),
    (M,m,m_source,m_active)
);

pub struct TupleProduct<T:IterTuple>{
    source_iters:T,
    active_iters:T,
    current_values:Option<T::Items>,
}
impl <T:IterTuple+Clone> TupleProduct<T>{
    // pub fn new(source_iters:T)->Self{
    //     let mut active_iters = source_iters.clone();
    //     let current_values = active_iters.get_nexts();
    //     Self { source_iters, active_iters, current_values }
    // }
}
impl <T:IterTuple> Iterator for TupleProduct<T> {
    type Item = T::Items;
    fn next(&mut self) -> Option<Self::Item> {
        IterTuple::iter_tuple_next(
            &mut self.source_iters,
            &mut self.active_iters,
            &mut self.current_values
        )
    }
}


pub struct ArrayProduct<const N:usize, I:Iterator>{
    source_iters:[I;N],
    active_iters:[I;N],
    current_values:Option<[I::Item;N]>,
}
impl <const N:usize, I:Iterator+Clone> ArrayProduct<N,I> {
    pub fn new<T:IntoIterator<IntoIter = I>>(source_iters:[T;N])->Self
        where
    {
        let source_iters = source_iters.map(IntoIterator::into_iter);
        let mut active_iters = source_iters.clone();
        let mut initial_values = [();N].map(|_|MaybeUninit::uninit());
        for (iter,value) in active_iters.iter_mut().zip(&mut initial_values) {
            if let Some(initial_value) = iter.next() {
                value.write(initial_value);
            }else {
                return Self {
                    source_iters,
                    active_iters,
                    current_values: None,
                }
            }
        };
        Self {
            source_iters,
            active_iters,
            current_values: Some(
                initial_values.map(|v|unsafe { v.assume_init()})
            )
        }
    }
}

impl <const N:usize, I:Iterator+Clone> Iterator for ArrayProduct<N,I>
    where I::Item:Clone
{
    type Item = [I::Item;N];
    fn next(&mut self)->Option<Self::Item>{
        let current_values = self.current_values.as_mut()?;
        let to_return = current_values.clone();
        let mut iter_and_items = self.source_iters.iter().zip(self.active_iters.iter_mut().zip(current_values));
        let item_to_increment = iter_and_items.try_fold((),|_,(source_iter,(iter,item))|{
            if let Some(next_item) = iter.next() { // If this slot can keep going
                ControlFlow::Break(Some((item,next_item)))
            }else {
                *iter = source_iter.clone().into_iter(); // Restart the slot
                if let Some(replacement_item) = iter.next() {
                    *item = replacement_item;// And store its first item
                    ControlFlow::Continue(())
                } else { // If for some reason this slot now doesn't have any items
                    ControlFlow::Break(None) // Break without an item to increment
                }
            }
        });
        match item_to_increment {
            // If a slot was found
            ControlFlow::Break(Some((item,new_item))) => {
                *item = new_item; // Update it
                Some(to_return) // And return the previous_state
            },
            // If no could be incremented
            _ => {
                // Then we looped back to the start!
                self.current_values = None; // Take away the current value
                Some(to_return) // And return the last state
            },
        }
        // while let Some((source_iter,(iter,item))) = iter_and_items.next(){
        //     if let Some(next_item)= iter.next() { // If this slot can keep going
        //         *item = next_item; // Increment it
        //         return self.current_values.clone(); // And return the current state
        //     }else{ // But if this slot is exhausted
        //         iter.clone_from(source_iter); // Restart the slot
        //         if let Some(replacement_item) = iter.next() {
        //             *item = replacement_item;// And store its first item
        //         } else { // If for some reason this slot now doesn't have any items
        //             self.current_values = None; // Then there are no more current values
        //             return None; // And no next item
        //         }
        //     }
        // }
        // None
    }
}
#[cfg(test)]
mod test {
    use super::ArrayProduct;
    #[test]
    fn bools(){
        let a = [false,true].into_iter();
        let b = {
            let mut iter = [false,true].into_iter();
            iter.next();
            iter
        };
        dbg!(ArrayProduct::new([a,b]).collect::<Vec<_>>());
    }
    #[test]
    fn tup(){
        // dbg!(TupleProduct::new((std::iter::empty::<usize>(),IntoIterator::into_iter([0,1,2]),IntoIterator::into_iter([false,true]))).collect::<Vec<_>>());
    }
}