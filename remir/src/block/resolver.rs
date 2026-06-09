//! The variable resolver for blocks.
//! Is used to sync variables from a block into another.
//!
//! This resolver might automatically get called in the future
//!

use crate::{
    block::{Block, BlockReference},
    builders::build_phi,
    errs::RemirResult,
    module::Module,
    values::BaseSSAValue,
};

impl Block {
    /// Resolves the variable at the given place.
    /// **Warn: This should be at the start
    pub fn resolve_variables(&mut self, module: &mut Module) -> RemirResult<()> {
        let mut vals = vec![];

        for (name, var) in self.variables.iter() {
            let mut choices: Vec<(BlockReference, BaseSSAValue)> = vec![];

            for block_ref in &self.origins {
                let block = &module.blocks[block_ref.id];

                if !block.variables.contains_key(name) {
                    continue;
                }

                let block_var = &block.variables[name];

                if block_var.held_value.is_none() {
                    continue;
                }

                unsafe {
                    choices.push((
                        block_ref.clone(),
                        block_var.held_value.clone().unwrap_unchecked(),
                    ))
                }
            }

            vals.push((name.clone(), choices));
        }

        for val in vals {
            if val.1.is_empty() {
                continue;
            }

            let res = build_phi(module, val.1)?;

            self.variables.get_mut(&val.0).unwrap().held_value = Some(res);
        }

        Ok(())
    }
}
