//! Formatting for instructions

use std::fmt::Display;

use crate::{block::BlockInstruction, fmt::utils::fmt_list, insts::Instruction, values::ValueType};

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstInt { val, size, signed } => {
                write!(
                    f,
                    "constint v{} t{}",
                    val,
                    ValueType::new_int(*signed, *size)
                )
            }

            Self::ConstFloat { val, size } => {
                write!(f, "constfloat v{} t{}", val, ValueType::new_float(*size))
            }

            Self::ConstPointer { addr } => {
                write!(f, "constptr v{}", addr)
            }

            Self::ConstString { str } => write!(f, "conststr v'{}'", str),

            Self::ConstStruct { values, ty } => {
                write!(f, "conststruct t{}, [{}]", ty, fmt_list(&values))
            }

            Self::ConstArray { values } => {
                write!(f, "constarr [{}]", fmt_list(values))
            }

            Self::ConstArraySame { value, count } => {
                write!(f, "constarr [{}]x{}", value, count)
            }

            Self::Copy { val } => {
                write!(f, "copy {}", val)
            }

            Self::Load { source } => {
                write!(f, "load {}", source)
            }

            Self::Store {
                destination,
                source,
            } => {
                write!(f, "store d{} s{}", destination, source)
            }

            Self::Not { val } => write!(f, "not {}", val),

            Self::MathOperationInt {
                a,
                b,
                op,
                signed,
                signed_wrap,
                unsigned_wrap,
                fast,
            } => {
                write!(f, "imathop {} {} op{}", a, b, op)?;

                if *signed {
                    write!(f, " signed")?;
                }

                if *signed_wrap {
                    write!(f, " sw")?;
                } else {
                    write!(f, " nsw")?;
                }

                if *unsigned_wrap {
                    write!(f, " uw")?;
                } else {
                    write!(f, " nuw")?;
                }

                if *fast {
                    write!(f, " fast")?;
                }

                Ok(())
            }

            Instruction::MathOperationFloat {
                a,
                b,
                op,
                signed_wrap,
                unsigned_wrap,
                fast,
            } => {
                write!(f, "fmathop {} {} op{}", a, b, op)?;

                if *signed_wrap {
                    write!(f, " sw")?;
                } else {
                    write!(f, " nsw")?;
                }

                if *unsigned_wrap {
                    write!(f, " uw")?;
                } else {
                    write!(f, " nuw")?;
                }

                if *fast {
                    write!(f, " fast")?;
                }

                Ok(())
            }

            Instruction::CompareOperationInt { a, b, op, signed } => {
                write!(f, "icmp {} {} op{}", a, b, op)?;

                if *signed {
                    write!(f, " signed")?;
                }

                Ok(())
            }

            Instruction::CompareOperationFloat { a, b, op } => {
                write!(f, "icmp {} {} op{}", a, b, op)
            }

            Instruction::UncondBr { branch } => write!(f, "uncondbr {}", branch),
            Instruction::Condbr {
                cond,
                true_label,
                false_label,
            } => {
                write!(f, "condbr c{} t{} f{}", cond, true_label, false_label)
            }

            Instruction::IndirectBranch {
                target,
                destinations: _,
            } => {
                write!(f, "indbranch {}", target)
            }

            Instruction::Phi { label_set } => {
                write!(f, "phi [ ")?;

                for label in label_set {
                    write!(f, "{}:{} ", label.0, label.1)?;
                }

                write!(f, "]")
            }

            Instruction::Call {
                func_label,
                args,
                pure,
                no_return,
                fast_calling_conv,
            } => {
                write!(f, "call {} [ ", func_label)?;

                for arg in args {
                    write!(f, "{} ", arg)?;
                }

                write!(f, "]")?;

                if *pure {
                    write!(f, " pure")?;
                }

                if *no_return {
                    write!(f, "noret")?;
                }

                if *fast_calling_conv {
                    write!(f, "fast")?;
                }

                Ok(())
            }

            Instruction::RetNull => write!(f, "ret"),
            Instruction::Ret { val } => write!(f, "ret {}", val),

            Instruction::GrabArgument { index } => write!(f, "garg {}", index),

            Instruction::Alloc { size, val_type } => write!(f, "heapalloc {} t{}", size, val_type),
            Instruction::AllocUntyped { size } => write!(f, "heapalloc {}", size),

            Instruction::Alloca { size, val_type } => {
                write!(f, "stackalloc {} t{}", size, val_type)
            }

            Instruction::AllocaUntyped { size } => write!(f, "stackalloc {}", size),
            Instruction::Free { ptr } => write!(f, "free {}", ptr),

            Instruction::Gep { base, offset } => write!(f, "gep b{} o{}", base, offset),
            Instruction::GepStruct { base, field } => write!(f, "structgep b{} o{}", base, field),
            Instruction::LoadIndexed { base, index } => write!(f, "indload b{} i{}", base, index),
            Instruction::StoreIndexed { base, index, val } => {
                write!(f, "indstore b{} i{} v{}", base, index, val)
            }

            Instruction::BitCast { src, into } => write!(f, "bitcast {} {}", src, into),
            Instruction::Select {
                cond,
                true_val,
                false_val,
            } => write!(f, "select {} t{} f{}", cond, true_val, false_val),

            Instruction::FloatToInt { val, into } => write!(f, "fti {} {}", val, into),
            Instruction::IntExtend { val, into } => write!(f, "inte {} {}", val, into),
            Instruction::IntTruncate { val, into } => write!(f, "intt {} {}", val, into),
            Instruction::IntToFloat { val, into } => write!(f, "itf {} {}", val, into),
            Instruction::FloatExtend { val, into } => write!(f, "floate {} {}", val, into),
            Instruction::FloatTruncate { val, into } => write!(f, "floatt {} {}", val, into),

            Instruction::ExtractValue { struct_val, index } => {
                write!(f, "extract {} i{}", struct_val, index)
            }

            Instruction::InsertValue {
                struct_val,
                index,
                val,
            } => write!(f, "insert {} i{} {}", struct_val, index, val),

            Instruction::Switch {
                value,
                else_block,
                cases,
            } => {
                write!(f, "switch {} {} [ ", value, else_block)?;

                for case in cases {
                    write!(f, "{}:{} ", case.0, case.1)?;
                }

                write!(f, " ]")
            }

            Instruction::LoadAtomic { source, ordering } => {
                write!(f, "atomload {} {}", source, ordering)
            }
            Instruction::StoreAtomic {
                dest,
                val,
                ordering,
            } => write!(f, "atomstore d{} s{} {}", dest, val, ordering),

            Instruction::Unreachable => write!(f, "unreachable"),
            Instruction::Crash { message } => {
                write!(f, "crash")?;

                if message.is_some() {
                    write!(f, " {}", message.clone().unwrap())?;
                }

                Ok(())
            }

            Instruction::Assume { val } => write!(f, "assume {}", val),

            Instruction::LazyLoad {
                block,
                variable_name,
                ty,
            } => write!(f, "lazyload b{} v{} t{}", block, variable_name, ty),
        }
    }
}

impl Display for BlockInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.is_some() {
            write!(
                f,
                "#{} = {}",
                self.value.as_ref().unwrap().inst_ind,
                &self.instruction
            )
        } else {
            write!(f, "{}", &self.instruction)
        }
    }
}
