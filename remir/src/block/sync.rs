//! The remir variable synchronizer.
//! Allows to sync variables between blocks correctly automatically.

use crate::{
    block::{Block, BlockReference},
    builders::build_lazy_load,
    module::Module,
    writer::InstructionWriter,
};

/// Represents a variable synchronizer core.
/// One common implementation is [`Module`][`crate::module::Module`]
///
/// # Sync point
/// The sync point represents the block from which variables will be copied.
///
/// As long as there is a sync point active, every new block will inherit the sync point's variables as well.
///
/// # Resolving
/// This does not handle variable resolving / updating using phi nodes in merge nodes.
/// In order to perform this, use the variable resolver: [`Block::resolve_variables`][`crate::block::Block::resolve_variables`].
///
pub trait VariableSynchronizer {
    /// Sets the variable sync point to the given block.
    fn set_sync_point(&mut self, block: BlockReference);

    /// Gets the current variable sync point.
    fn get_sync_point(&self) -> Option<BlockReference>;

    /// Removes the variable sync point.
    fn stop_sync_point(&mut self);

    /// Inherits the variables of the sync point onto the given block
    fn inherit_sync_point(&mut self, block: &mut Block);
}

impl VariableSynchronizer for Module {
    #[inline]
    fn set_sync_point(&mut self, block: BlockReference) {
        self.variable_sync_point = Some(block);
    }

    #[inline]
    fn stop_sync_point(&mut self) {
        self.variable_sync_point = None;
    }

    #[inline]
    fn get_sync_point(&self) -> Option<BlockReference> {
        self.variable_sync_point.clone()
    }

    #[inline]
    fn inherit_sync_point(&mut self, block: &mut Block) {
        if self.get_sync_point().is_none() {
            return;
        }

        let curr_block = self.pos_block.clone();
        let curr_state = self.pos_is_start;

        let variables = self.blocks[self.get_sync_point().unwrap().id]
            .variables
            .clone();
        let reference = self.blocks[self.get_sync_point().unwrap().id]
            .reference
            .clone();

        self.move_end(block.reference.clone(), self.pos_function.clone().unwrap());

        for (key, value) in variables {
            let lazy_value = build_lazy_load(
                self,
                reference.clone(),
                key.clone(),
                value.held_value.as_ref().unwrap().value_type.clone(),
            )
            .unwrap();

            block.append_variable(value.with_value(Some(lazy_value)));
        }

        self.pos_block = curr_block;
        self.pos_is_start = curr_state;
    }
}
