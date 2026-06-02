# Remir
Remir is an MIR library designed to be used easily and be efficient. 
Currently Remir supports LLVM but GCC support will be added in the future.

This was originally made for [Calscin](https://github.com/calscin/calscin) but was uploaded as another project of mine for easier issue tracking.

## Safety
Remir enforces a strict typing equivalence for every instruction and value. Values obtained from instructions hold a type, this type represent the type of the value itself. This allows for automatic signed handling and safety. Furthermore, every builder function enforces that the value types are correct for the given instruction. 

## Representation
Remir uses a block representation with instructions contained in modules. This representation is similar to LLVM's IR. Remir's representation is designed to be low level. 
However, there are some helpers (eg: `BlockVariable`) that can allow to handle raw instructions for certain concepts 

For raw instructions, Remir implements builders. Builders are used to enforce that the values given are valid for the given instruction.

## Capabilities:
Currently, Remir has instructions for most basic tasks. You should be able to perform mostly everything with Remir. However, atomic instructions are currently limited and will be expanded in the future when needed for Calscin. 