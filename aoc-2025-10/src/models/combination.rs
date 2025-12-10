use std::{cmp::Reverse, collections::BinaryHeap};

use super::{Button, CountArray, Difference, PartialOrdered};

pub struct ButtonCombination<'b> {
    buttons: &'b [Button],
    target: &'b CountArray<u16>,
    pub(crate) difference: Difference<i16>,
    pub(crate) combination: Vec<usize>,
    heap: BinaryHeap<Reverse<PartialOrdered<(usize, CountArray<u16>)>>>,
}

impl<'b> ButtonCombination<'b> {
    pub fn new(
        buttons: &'b [Button],
        target: &'b CountArray<u16>,
        combination: Vec<usize>,
    ) -> anyhow::Result<Self> {
        // Create the initial state in order to calculate distance.
        let init_state = Button::combine(combination.iter().map(|id| &buttons[*id]), target.len())?;
        #[cfg(feature = "trace")]
        eprintln!(
            "Creating ButtonCombination with target: {:?}, combination: {:?}, state: {:?}",
            target.values, combination, init_state.values,
        );
        let difference = target.difference_from(&init_state);

        Self::new_with_init_state_and_difference(
            buttons,
            target,
            combination,
            init_state,
            difference,
        )
    }

    fn new_with_init_state_and_difference(
        buttons: &'b [Button],
        target: &'b CountArray<u16>,
        combination: Vec<usize>,
        init_state: CountArray<u16>,
        difference: Difference<i16>,
    ) -> anyhow::Result<Self> {
        let mut instance = Self {
            buttons,
            target,
            difference,
            combination,
            heap: BinaryHeap::with_capacity(buttons.len()),
        };

        // Only initialize the heap if we have not
        // - found the solution, or
        // - exceeded the target.
        if let &Difference::Incomplete(_) = &instance.difference {
            instance.init(init_state)?;
        }

        Ok(instance)
    }

    pub fn solution(&self) -> Option<&Vec<usize>> {
        if matches!(self.difference, Difference::Equal) {
            Some(&self.combination)
        } else {
            None
        }
    }

    pub fn is_dead_end(&self) -> bool {
        matches!(self.difference, Difference::Overshot(_))
    }

    pub fn len(&self) -> usize {
        self.target.len()
    }

    fn init(&mut self, init_state: CountArray<u16>) -> anyhow::Result<()> {
        self.buttons
            .iter()
            .try_for_each(|next_button| -> anyhow::Result<()> {
                let new_state = init_state.add(&next_button.effect)?;

                self.heap.push(Reverse(PartialOrdered {
                    difference: self.target.difference_from(&new_state),
                    data: (next_button.index, new_state),
                }));

                Ok(())
            })?;

        Ok(())
    }

    pub fn next_combination(&mut self) -> Option<anyhow::Result<ButtonCombination<'b>>> {
        self.heap.pop().map(
            |Reverse(PartialOrdered {
                 difference,
                 data: (index, init_state),
             })| {
                let mut combination = self.combination.clone();
                combination.push(index);

                ButtonCombination::new_with_init_state_and_difference(
                    self.buttons,
                    self.target,
                    combination,
                    init_state,
                    difference,
                )
            },
        )
    }
}
