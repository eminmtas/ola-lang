use super::{get_operand_for_val, get_operands_for_val, new_empty_str_inst_output};
use crate::codegen::core::ir::{function::instruction::InstructionId, types::Type, value::ValueId};
use crate::codegen::{
    function::instruction::Instruction as MachInstruction,
    isa::ola::{
        instruction::{InstructionData, Opcode, Operand as MO, OperandData},
        Ola,
    },
    lower::{LoweringContext, LoweringError},
};
use anyhow::Result;

pub fn lower_insertvalue(
    ctx: &mut LoweringContext<Ola>,
    id: InstructionId,
    tys: &[Type],
    args: &[ValueId],
) -> Result<()> {
    println!("lower insertvalue");
    let value = get_operands_for_val(ctx, tys[0], args[0])?;
    let op_value = get_operand_for_val(ctx, tys[1], args[1])?;
    let elm_ty = ctx.types.base().element(tys[0]).unwrap();
    let op_idx = get_operand_for_val(ctx, elm_ty, args[2])?;

    let idx = match op_idx {
        OperandData::Int32(op_idx) => op_idx,
        e => {
            return Err(LoweringError::Todo(format!(
                "Unsupported inserttvalue idx operand: {:?}",
                e
            ))
            .into())
        }
    };
    //let mut output = vec![];
    let output = new_empty_str_inst_output(ctx, elm_ty, id);
    for ist_idx in 0..4 {
        let input = if ist_idx == idx {
            op_value.clone()
        } else {
            value[ist_idx as usize].clone()
        };
        ctx.inst_seq.push(MachInstruction::new(
            InstructionData {
                opcode: Opcode::MOVrr,
                operands: vec![
                    MO::output(output[ist_idx as usize].into()),
                    MO::input(input),
                ],
            },
            ctx.block_map[&ctx.cur_block],
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::codegen::{
        core::ir::module::Module,
        isa::ola::{asm::AsmProgram, Ola},
        lower::compile_module,
    };
    #[test]
    fn codegen_insertv_extractv_test() {
        // LLVM Assembly
        let asm = r#"
; ModuleID = 'StrImm'
source_filename = "examples/source/storage/storage_u32.ola"

declare void @builtin_assert(i64, i64)

declare void @builtin_range_check(i64)

declare i64 @prophet_u32_sqrt(i64)

declare i64 @prophet_u32_div(i64, i64)

declare i64 @prophet_u32_mod(i64, i64)

declare ptr @prophet_u32_array_sort(ptr, i64)

declare ptr @vector_new(i64, ptr)

declare [4 x i64] @get_storage([4 x i64])

declare void @set_storage([4 x i64], [4 x i64])

declare [4 x i64] @poseidon_hash([8 x i64])

define i64 @insert(i64 %0) {
    entry:
      %a = alloca i64, align 8
      store i64 %0, ptr %a, align 4
      %1 = load i64, ptr %a, align 4
      %2 = add i64 %1, 1
      call void @builtin_range_check(i64 %2)
      %3 = insertvalue [4 x i64] [i64 10, i64 20, i64 30, i64 40], i64 %2, 2
      %4 = extractvalue [4 x i64] %3, 2
      %5 = add i64 %4, 2
      call void @set_storage([4 x i64] [i64 100, i64 200, i64 300, i64 400], [4 x i64] [i64 500, i64 600, i64 700, i64 800])
      ret i64 %5
    }
"#;

        // Parse the assembly and get a module
        let module = Module::try_from(asm).expect("failed to parse LLVM IR");

        // Compile the module for Ola and get a machine module
        let isa = Ola::default();
        let mach_module = compile_module(&isa, &module).expect("failed to compile");

        // Display the machine module as assembly
        let code: AsmProgram =
            serde_json::from_str(mach_module.display_asm().to_string().as_str()).unwrap();
        println!("{}", code.program);
        assert_eq!(
            format!("{}", code.program),
            "insert:
.LBL10_0:
  add r9 r9 2
  mstore [r9,-1] r1
  mload r1 [r9,-1]
  add r0 r1 1
  range r0
  mov r1 100
  mov r2 200
  mov r3 300
  mov r4 400
  mov r5 500
  mov r6 600
  mov r7 700
  mov r8 800
  sstore 
  mov r1 10
  mov r2 20
  mov r3 30
  mov r3 40
  mov r1 r2
  mov r1 r3
  add r0 r0 2
  mstore [r9,-2] r0
  mload r0 [r9,-2]
  add r9 r9 -2
  ret
"
        );
    }

    #[test]
    fn codegen_insertv_vec_test() {
        // LLVM Assembly
        let asm = r#"
; ModuleID = 'StrImm'
source_filename = "examples/source/storage/storage_u32.ola"

declare void @builtin_assert(i64, i64)

declare void @builtin_range_check(i64)

declare i64 @prophet_u32_sqrt(i64)

declare i64 @prophet_u32_div(i64, i64)

declare i64 @prophet_u32_mod(i64, i64)

declare ptr @prophet_u32_array_sort(ptr, i64)

declare ptr @vector_new(i64, ptr)

declare [4 x i64] @get_storage([4 x i64])

declare void @set_storage([4 x i64], [4 x i64])

declare [4 x i64] @poseidon_hash([8 x i64])

define void @insert(i64 %0) {
    entry:
      %a = alloca i64, align 8
      store i64 %0, ptr %a, align 4
      %1 = load i64, ptr %a, align 4
      %2 = add i64 %1, 1
      call void @builtin_range_check(i64 %2)
      %3 = insertvalue [4 x i64] [i64 10, i64 20, i64 30, i64 40], i64 %2, 2
      call void @set_storage([4 x i64] zeroinitializer, [4 x i64] %3)
      ret void
    }
"#;

        // Parse the assembly and get a module
        let module = Module::try_from(asm).expect("failed to parse LLVM IR");

        // Compile the module for Ola and get a machine module
        let isa = Ola::default();
        let mach_module = compile_module(&isa, &module).expect("failed to compile");

        // Display the machine module as assembly
        let code: AsmProgram =
            serde_json::from_str(mach_module.display_asm().to_string().as_str()).unwrap();
        println!("{}", code.program);
        assert_eq!(
            format!("{}", code.program),
            "insert:
.LBL10_0:
  add r9 r9 1
  mov r0 r1
  mstore [r9,-1] r0
  mload r0 [r9,-1]
  add r7 r0 1
  range r7
  mov r1 0
  mov r2 0
  mov r3 0
  mov r4 0
  mov r5 10
  mov r6 20
  mov r0 30
  mov r8 40
  sstore 
  add r9 r9 -1
  ret
"
        );
    }
}
