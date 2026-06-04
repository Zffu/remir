use std::fmt::Display;

use crate::{fmt::utils::fmt_list, func::Function, values::ValueType};

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#val_ind_ctr: {}", self.value_index_counter)?;

        if self.return_type.is_some() {
            writeln!(f, "#ret_type {}", self.return_type.as_ref().unwrap())?;
        }

        if !self.arguments.is_empty() {
            let types: Vec<&ValueType> = self.arguments.iter().collect();

            writeln!(f, "#arguments [{}]", fmt_list(&types))?;
        }

        if !self.blocks.is_empty() {
            writeln!(f, "#blocks [{}]", fmt_list(&self.blocks))?;
        }

        writeln!(f, "{}", self.reference)
    }
}
