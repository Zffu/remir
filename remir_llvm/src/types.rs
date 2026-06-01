use std::{collections::HashMap, mem::transmute, num::NonZero, rc::Rc};

use inkwell::{AddressSpace, context::Context, types::BasicTypeEnum};
use remir::values::ValueType;

use crate::utils::LLVMTypeEnum;

pub struct LLVMTypeStorage {
    pub map: HashMap<ValueType, LLVMTypeEnum>,
    pub ctx_ref: Rc<Context>,
}

impl LLVMTypeStorage {
    pub fn new(ctx_ref: &Rc<Context>) -> Self {
        LLVMTypeStorage {
            map: HashMap::new(),
            ctx_ref: ctx_ref.clone(),
        }
    }

    pub fn convert(&mut self, base: ValueType) -> LLVMTypeEnum {
        if self.map.contains_key(&base) {
            return LLVMTypeEnum::clone(&self.map[&base]);
        }

        let conv: BasicTypeEnum = match &base {
            ValueType::Int(_, size) => self
                .ctx_ref
                .custom_width_int_type(NonZero::new(*size as u32).unwrap())
                .unwrap()
                .into(),

            ValueType::Float(size) => match *size {
                16 => self.ctx_ref.f16_type().into(),
                32 => self.ctx_ref.f32_type().into(),
                64 => self.ctx_ref.f64_type().into(),
                80 => self.ctx_ref.x86_f80_type().into(),
                128 => self.ctx_ref.f128_type().into(),

                _ => panic!(),
            },

            ValueType::Pointer(_) => self.ctx_ref.ptr_type(AddressSpace::from(0)).into(), // TODO: add address space in Remir layer

            ValueType::Struct(fields) => {
                let mut fs = vec![];

                for field in fields {
                    fs.push(self.convert(*field.clone()).inner);
                }

                self.ctx_ref.struct_type(&fs, false).into()
            }

            _ => panic!(),
        };

        let l = LLVMTypeEnum::new(
            unsafe { transmute::<BasicTypeEnum, BasicTypeEnum<'static>>(conv) },
            &self.ctx_ref,
        );

        self.map.insert(base, l.clone());

        l
    }
}
