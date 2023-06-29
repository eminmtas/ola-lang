#[macro_use]
pub mod call_conv;
pub mod core;
pub mod function;
pub mod isa;
pub mod lower;
pub mod module;
pub mod pass;
pub mod register;

#[cfg(test)]
mod test {
    use crate::codegen::{
        core::ir::module::Module,
        isa::ola::{asm::AsmProgram, Ola},
        lower::compile_module,
    };
    #[test]
    fn codegen_binop_test() {
        // LLVM Assembly
        let asm = r#"
  source_filename = "asm"
  ; Function Attrs: noinline nounwind optnone uwtable
  define dso_local i32 @main() #0 {
    %a = alloca i32, align 4
    store i32 10, i32* %a
    %b = load i32, i32* %a
    %c = add i32 %b, 20 ; 30
    %d = add i32 %b, 30 ; 60
    %e = mul i32 %c, %d ; 1800
    %f = sub i32 %e, %d ; 1740
    ret i32 %f
  }
  attributes #0 = { noinline nounwind optnone uwtable }
"#;

        // Parse the assembly and get a module
        let module = Module::try_from(asm).expect("failed to parse LLVM IR");

        // Compile the module for Ola and get a machine module
        let isa = Ola::default();
        let mach_module = compile_module(&isa, &module).expect("failed to compile");

        // Display the machine module as assembly
        let code: AsmProgram =
            serde_json::from_str(mach_module.display_asm().to_string().as_str()).unwrap();
        assert_eq!(
            format!("{}", code.program),
            "main:
.LBL0_0:
  add r9 r9 1
  mov r4 10
  mstore [r9,-1] r4
  mload r4 [r9,-1]
  add r0 r4 20
  add r1 r4 30
  mul r2 r0 r1
  not r7 r1
  add r7 r7 1
  add r3 r2 r7
  mov r0 r3
  add r9 r9 -1
  end
"
        );
    }

    #[test]
    fn codegen_storage_contract_test() {
        // LLVM Assembly
        let asm = r#"
; ModuleID = 'SimpleVar'
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

define void @inc_simple() {
entry:
  call void @set_storage([4 x i64] zeroinitializer, [4 x i64] [i64 100, i64 200, i64 300, i64 400])
  ret void
}

define i64 @get() {
entry:
  %0 = call [4 x i64] @get_storage([4 x i64] zeroinitializer)
  %1 = extractvalue [4 x i64] %0, 2
  %2 = extractvalue [4 x i64] %0, 3
  %3 = add i64 %1, %2
  ret i64 %3
}

define void @main() {
entry:
  %x = alloca i64, align 8
  call void @inc_simple()
  %0 = call i64 @get()
  store i64 %0, ptr %x, align 4
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
            "inc_simple:
.LBL10_0:
  mov r1 0
  mov r2 0
  mov r3 0
  mov r4 0
  mov r5 100
  mov r6 200
  mov r7 300
  mov r8 400
  sstore 
  ret
get:
.LBL11_0:
  mov r1 0
  mov r2 0
  mov r3 0
  mov r4 0
  sload 
  mov r1 r2
  mov r1 r3
  mov r2 r4
  add r0 r1 r2
  ret
main:
.LBL12_0:
  add r9 r9 5
  mstore [r9,-2] r9
  call inc_simple
  call get
  mstore [r9,-3] r0
  add r9 r9 -5
  end
"
        );
    }

    #[test]
    fn codegen_str_u32_imm_test() {
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

define void @str_imm(i64 %0) {   ;mstore [r9,-1] r1 
entry:
  %a = alloca i64, align 8      ;[r9,-2]
  store i64 %0, ptr %a, align 4     ;mload r0 [r9,-1]  mstore [r9,-2] r0  
  %1 = load i64, ptr %a, align 4    ;mload r0 [r9,-2] 
  ;%2 = insertvalue [4 x i64] [i64 0, i64 0, i64 0, i64 undef], i64 %1, 3    ;mov r1 0, mov r2 0, mov r3 0,mov r4 r0
  ;call void @set_storage([4 x i64] zeroinitializer, [4 x i64] %2)   ;mov r5 0,mov r6 0,mov r7 0,mov r8 0
  ;call void @set_storage([4 x i64] [i64 1, i64 2, i64 3, i64 4], [4 x i64] [i64 5, i64 6, i64 7, i64 8])
  call void @set_storage([4 x i64] zeroinitializer, [4 x i64] [i64 5, i64 6, i64 7, i64 8])
  %3 = call [4 x i64] @get_storage([4 x i64] [i64 1, i64 2, i64 3, i64 4])
  call void @set_storage([4 x i64] zeroinitializer, [4 x i64] %3)
  %4 = call [4 x i64] @poseidon_hash([8 x i64] [i64 10, i64 20, i64 30, i64 40, i64 50, i64 60, i64 70, i64 80])
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
            "str_imm:
.LBL10_0:
  add r9 r9 1
  mov r0 r1
  mstore [r9,-1] r0
  mload r0 [r9,-1]
  mov r1 0
  mov r2 0
  mov r3 0
  mov r4 0
  mov r5 5
  mov r6 6
  mov r7 7
  mov r8 8
  sstore 
  mov r1 1
  mov r2 2
  mov r3 3
  mov r4 4
  sload 
  mov r5 r1
  mov r6 r2
  mov r7 r3
  mov r8 r4
  mov r1 0
  mov r2 0
  mov r3 0
  mov r4 0
  sstore 
  mov r1 10
  mov r2 20
  mov r3 30
  mov r4 40
  mov r5 50
  mov r6 60
  mov r7 70
  mov r8 80
  poseidon 
  add r9 r9 -1
  ret
"
        );
    }

    #[test]
    fn codegen_str_u32_var_test() {
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

define void @str_var(i64 %0) {   ;mstore [r9,-1] r1 
entry:
  %3 = call [4 x i64] @get_storage([4 x i64] [i64 1, i64 2, i64 3, i64 4])
  call void @set_storage([4 x i64] zeroinitializer, [4 x i64] %3)
  %4 = call [4 x i64] @poseidon_hash([8 x i64] [i64 10, i64 20, i64 30, i64 40, i64 50, i64 60, i64 70, i64 80])
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
            "str_var:
.LBL10_0:
  mov r0 r1
  mov r1 1
  mov r2 2
  mov r3 3
  mov r4 4
  sload 
  mov r5 r1
  mov r6 r2
  mov r7 r3
  mov r8 r4
  mov r1 0
  mov r2 0
  mov r3 0
  mov r4 0
  sstore 
  mov r1 10
  mov r2 20
  mov r3 30
  mov r4 40
  mov r5 50
  mov r6 60
  mov r7 70
  mov r8 80
  poseidon 
  ret
"
        );
    }

    #[test]
    fn codegen_str_poseidon_test() {
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

define void @str_var(i64 %0) {   ;mstore [r9,-1] r1 
entry:
  %3 = call [4 x i64] @get_storage([4 x i64] [i64 1, i64 2, i64 3, i64 4])
  call void @set_storage([4 x i64] zeroinitializer, [4 x i64] %3)
  %4 = call [4 x i64] @poseidon_hash([8 x i64] [i64 10, i64 20, i64 30, i64 40, i64 50, i64 60, i64 70, i64 80])
  call void @set_storage([4 x i64] [i64 5, i64 6, i64 7, i64 8], [4 x i64] %4)
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
            "str_var:
.LBL10_0:
  mov r0 r1
  mov r1 1
  mov r2 2
  mov r3 3
  mov r4 4
  sload 
  mov r5 r1
  mov r6 r2
  mov r7 r3
  mov r8 r4
  mov r1 0
  mov r2 0
  mov r3 0
  mov r4 0
  sstore 
  mov r1 10
  mov r2 20
  mov r3 30
  mov r4 40
  mov r5 50
  mov r6 60
  mov r7 70
  mov r8 80
  poseidon 
  mov r5 r1
  mov r6 r2
  mov r7 r3
  mov r8 r4
  mov r1 5
  mov r2 6
  mov r3 7
  mov r4 8
  sstore 
  ret
"
        );
    }

    #[ignore]
    #[test]
    fn codegen_str_binop_test() {
        // LLVM Assembly
        let asm = r#"
  source_filename = "asm"
  ; Function Attrs: noinline nounwind optnone uwtable
  define dso_local i32 @main() #0 {
    %a = alloca i32, align 4
    store i32 10, i32* %a
    %b = load i32, i32* %a
    %c = add i32 %b, 20 ; 30
    %d = add i32 %b, 30 ; 60
    %e = mul i32 %c, %d ; 1800
    ;%f = sub i32 %e, %d ; 1740
    ;call void @ordinary_call(i64 100,i64 %b, i64 %e)
    %d1 = add i32 %d, 1000
    %d2 = add i32 %d1, 2000
    %d3 = add i32 %d2, 3000
    %d4 = add i32 %d3, 4000
    %d5 = add i32 %d4, 5000
    %d6 = add i32 %d5, 6000
    %d7 = add i32 %d5, %d6
    %d8 = add i32 %d5, %d7
    %d9 = add i32 %d7, %d8
    call void @set_storage(i64 %d9, i64 %b, i64 2, i64 3, i64 4, i64 5, i64 6, i64 %e)
    ;call void @get_storage(i64 10, i64 20, i64 30, i64 40, i64 50, i64 60, i64 70, i64 %c)
    %g = add i32 %d9, 666
    ret i32 %g
  }
  attributes #0 = { noinline nounwind optnone uwtable }
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
            "main:
.LBL0_0:
  add r8 r8 12
  mov r1 10
  mstore [r8,-1] r1
  mload r1 [r8,-1]
  add r0 r1 20
  add r2 r1 30
  mstore [r8,-2] r2
  mload r2 [r8,-2]
  mul r7 r0 r2
  mov r6 6
  mov r5 5
  mov r4 4
  mov r3 3
  mov r2 2
  mload r0 [r8,-2]
  add r0 r0 1000
  mstore [r8,-3] r0
  mload r0 [r8,-3]
  add r0 r0 2000
  mstore [r8,-4] r0
  mload r0 [r8,-4]
  add r0 r0 3000
  mstore [r8,-5] r0
  mload r0 [r8,-5]
  add r0 r0 4000
  mstore [r8,-6] r0
  mload r0 [r8,-6]
  add r0 r0 5000
  mstore [r8,-8] r0
  mload r0 [r8,-8]
  add r0 r0 6000
  mstore [r8,-7] r0
  mload r0 [r8,-7]
  mload r1 [r8,-8]
  add r0 r1 r0
  mstore [r8,-9] r0
  mload r0 [r8,-8]
  mload r1 [r8,-9]
  add r0 r0 r1
  mstore [r8,-10] r0
  mload r0 [r8,-9]
  mload r1 [r8,-10]
  add r0 r0 r1
  mstore [r8,-11] r0
  mload r0 [r8,-11]
  sstore 
  mload r0 [r8,-11]
  add r0 r0 666
  mstore [r8,-12] r0
  mload r0 [r8,-12]
  add r8 r8 -12
  end
"
        );
    }

    #[ignore]
    #[test]
    fn codegen_functioncall_test() {
        // LLVM Assembly
        let asm = r#"
source_filename = "asm"

; Function Attrs: noinline nounwind optnone ssp uwtable
define void @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  store i32 10, i32* %1, align 4
  store i32 20, i32* %2, align 4
  store i32 100, i32* %3, align 4
  %4 = load i32, i32* %1, align 4
  %5 = load i32, i32* %2, align 4
  %6 = call i32 @bar(i32 %4, i32 %5)
  store i32 %6, i32* %3, align 4
  %7 = load i32, i32* %3, align 4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @bar(i32 %0, i32 %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  store i32 %0, i32* %3, align 4
  store i32 %1, i32* %4, align 4
  store i32 200, i32* %5, align 4
  %6 = load i32, i32* %3, align 4
  %7 = load i32, i32* %4, align 4
  %8 = add nsw i32 %6, %7
  store i32 %8, i32* %5, align 4
  %9 = load i32, i32* %5, align 4
  ret i32 %9
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
        assert_eq!(
            format!("{}", code.program),
            "main:
.LBL0_0:
  add r8 r8 7
  mstore [r8,-2] r8
  mov r0 10
  mstore [r8,-5] r0
  mov r0 20
  mstore [r8,-4] r0
  mov r0 100
  mstore [r8,-3] r0
  mload r1 [r8,-5]
  mload r2 [r8,-4]
  call bar
  mstore [r8,-3] r0
  mload r0 [r8,-3]
  add r8 r8 -7
  end
bar:
.LBL1_0:
  add r8 r8 3
  mstore [r8,-3] r1
  mstore [r8,-2] r2
  mov r1 200
  mstore [r8,-1] r1
  mload r1 [r8,-3]
  mload r2 [r8,-2]
  add r0 r1 r2
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  add r8 r8 -3
  ret
"
        );
    }

    #[ignore]
    #[test]
    fn codegen_functioncall_multiparams_test() {
        // LLVM Assembly
        let asm = r#"
source_filename = "asm"

; Function Attrs: noinline nounwind optnone ssp uwtable
define void @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 10, i32* %1, align 4
  store i32 20, i32* %2, align 4
  store i32 30, i32* %3, align 4
  store i32 40, i32* %4, align 4
  %5 = load i32, i32* %1, align 4
  %6 = load i32, i32* %2, align 4
  %7 = load i32, i32* %3, align 4
  %8 = load i32, i32* %4, align 4
  %9 = call i32 @add(i32 %5, i32 %6, i32 %7, i32 %8)
  store i32 %9, i32* %1, align 4
  %10 = load i32, i32* %1, align 4
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @bar(i32 %0, i32 %1, i32 %2, i32 %3) #0 {
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  %7 = alloca i32, align 4
  store i32 %0, i32* %4, align 4
  store i32 %1, i32* %5, align 4
  store i32 %2, i32* %6, align 4
  store i32 %3, i32* %7, align 4
  %8 = load i32, i32* %4, align 4
  %9 = load i32, i32* %5, align 4
  %10 = load i32, i32* %6, align 4
  %11 = load i32, i32* %7, align 4
  %12 = add nsw i32 %8, %9
  %13 = add nsw i32 %10, %11
  %14 = add nsw i32 %12, %13
  store i32 %14, i32* %4, align 4
  %15 = load i32, i32* %4, align 4
  ret i32 %15
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
        assert_eq!(
            format!("{}", code.program),
            "main:
.LBL0_0:
  add r8 r8 7
  mstore [r8,-2] r8
  mov r0 10
  mstore [r8,-5] r0
  mov r0 20
  mstore [r8,-4] r0
  mov r0 100
  mstore [r8,-3] r0
  mload r1 [r8,-5]
  mload r2 [r8,-4]
  call bar
  mstore [r8,-3] r0
  mload r0 [r8,-3]
  add r8 r8 -7
  end
bar:
.LBL1_0:
  add r8 r8 3
  mstore [r8,-3] r1
  mstore [r8,-2] r2
  mov r1 200
  mstore [r8,-1] r1
  mload r1 [r8,-3]
  mload r2 [r8,-2]
  add r0 r1 r2
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  add r8 r8 -3
  ret
"
        );
    }

    #[ignore]
    #[test]
    fn codegen_fib_test() {
        // LLVM Assembly
        let asm = r#"
    ; ModuleID = 'Fibonacci'
source_filename = "fib.ola"

define void @main() {
entry:
  %0 = call i32 @fib_non_recursive(i32 10)
  ret void
}

define i32 @fib_recursive(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  %1 = load i32, i32* %n, align 4
  %2 = icmp ule i32 %1, 2
  br i1 %2, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 1

enif:                                             ; preds = %entry
  %3 = load i32, i32* %n, align 4
  %4 = sub i32 %3, 1
  %5 = call i32 @fib_recursive(i32 %4)
  %6 = load i32, i32* %n, align 4
  %7 = sub i32 %6, 2
  %8 = call i32 @fib_recursive(i32 %7)
  %9 = add i32 %5, %8
  ret i32 %9
}

define i32 @fib_non_recursive(i32 %0) {
entry:
  %i = alloca i32, align 4
  %third = alloca i32, align 4
  %second = alloca i32, align 4
  %first = alloca i32, align 4
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  store i32 0, i32* %first, align 4
  store i32 1, i32* %second, align 4
  store i32 1, i32* %third, align 4
  store i32 2, i32* %i, align 4
  br label %cond

cond:                                             ; preds = %next, %entry
  %1 = load i32, i32* %i, align 4
  %2 = load i32, i32* %n, align 4
  %3 = icmp ule i32 %1, %2
  br i1 %3, label %body, label %endfor

body:                                             ; preds = %cond
  %4 = load i32, i32* %first, align 4
  %5 = load i32, i32* %second, align 4
  %6 = add i32 %4, %5
  store i32 %6, i32* %third, align 4
  %7 = load i32, i32* %second, align 4
  store i32 %7, i32* %first, align 4
  %8 = load i32, i32* %third, align 4
  store i32 %8, i32* %second, align 4
  br label %next

next:                                             ; preds = %body
  %9 = load i32, i32* %i, align 4
  %10 = add i32 %9, 1
  store i32 %10, i32* %i, align 4
  br label %cond

endfor:                                           ; preds = %cond
  %11 = load i32, i32* %third, align 4
  ret i32 %11
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
        assert_eq!(
            format!("{}", code.program),
            "main:
.LBL0_0:
  add r8 r8 4
  mstore [r8,-2] r8
  mov r1 10
  call fib_non_recursive
  add r8 r8 -4
  end
fib_recursive:
.LBL1_0:
  add r8 r8 9
  mstore [r8,-2] r8
  mov r0 r1
  mstore [r8,-7] r0
  mload r0 [r8,-7]
  mov r7 2
  gte r0 r7 r0
  cjmp r0 .LBL1_1
  jmp .LBL1_2
.LBL1_1:
  mov r0 1
  add r8 r8 -9
  ret
.LBL1_2:
  mload r0 [r8,-7]
  not r7 1
  add r7 r7 1
  add r1 r0 r7
  call fib_recursive
  mstore [r8,-3] r0
  mload r0 [r8,-7]
  not r7 2
  add r7 r7 1
  add r0 r0 r7
  mstore [r8,-5] r0
  mload r1 [r8,-5]
  call fib_recursive
  mload r1 [r8,-3]
  add r0 r1 r0
  mstore [r8,-6] r0
  mload r0 [r8,-6]
  add r8 r8 -9
  ret
fib_non_recursive:
.LBL2_0:
  add r8 r8 5
  mov r0 r1
  mstore [r8,-1] r0
  mov r0 0
  mstore [r8,-2] r0
  mov r0 1
  mstore [r8,-3] r0
  mov r0 1
  mstore [r8,-4] r0
  mov r0 2
  mstore [r8,-5] r0
  jmp .LBL2_1
.LBL2_1:
  mload r0 [r8,-5]
  mload r1 [r8,-1]
  gte r0 r1 r0
  cjmp r0 .LBL2_2
  jmp .LBL2_4
.LBL2_2:
  mload r1 [r8,-2]
  mload r2 [r8,-3]
  add r0 r1 r2
  mstore [r8,-4] r0
  mload r0 [r8,-3]
  mstore [r8,-2] r0
  mload r0 [r8,-4]
  mstore [r8,-3] r0
  jmp .LBL2_3
.LBL2_3:
  mload r1 [r8,-5]
  add r0 r1 1
  mstore [r8,-5] r0
  jmp .LBL2_1
.LBL2_4:
  mload r0 [r8,-4]
  add r8 r8 -5
  ret
"
        );
    }

    #[ignore]
    #[test]
    fn codegen_condbr_test() {
        // LLVM Assembly
        let asm = r#"
  source_filename = "test.ola"

define void @main() {
entry:
  %0 = call i32 @eq_rr(i32 1)
  ret void
}

define i32 @eq_ri(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  %1 = load i32, i32* %n, align 4
  %2 = icmp eq i32 %1, 1
  br i1 %2, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @eq_rr(i32 %0) {
entry:
  %m = alloca i32, align 4
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  store i32 1, i32* %m, align 4
  %1 = load i32, i32* %n, align 4
  %2 = load i32, i32* %m, align 4
  %3 = icmp eq i32 %1, %2
  br i1 %3, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @neq_ri(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  %1 = load i32, i32* %n, align 4
  %2 = icmp ne i32 %1, 1
  br i1 %2, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @neq_rr(i32 %0) {
entry:
  %m = alloca i32, align 4
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  store i32 1, i32* %m, align 4
  %1 = load i32, i32* %n, align 4
  %2 = load i32, i32* %m, align 4
  %3 = icmp ne i32 %1, %2
  br i1 %3, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @lt_ri(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  %1 = load i32, i32* %n, align 4
  %2 = icmp ult i32 %1, 1
  br i1 %2, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @lt_rr(i32 %0) {
entry:
  %m = alloca i32, align 4
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  store i32 1, i32* %m, align 4
  %1 = load i32, i32* %n, align 4
  %2 = load i32, i32* %m, align 4
  %3 = icmp ult i32 %1, %2
  br i1 %3, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @lte_ri(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  %1 = load i32, i32* %n, align 4
  %2 = icmp ule i32 %1, 1
  br i1 %2, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @lte_rr(i32 %0) {
entry:
  %m = alloca i32, align 4
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  store i32 1, i32* %m, align 4
  %1 = load i32, i32* %n, align 4
  %2 = load i32, i32* %m, align 4
  %3 = icmp ule i32 %1, %2
  br i1 %3, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @gt_ri(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  %1 = load i32, i32* %n, align 4
  %2 = icmp ugt i32 %1, 1
  br i1 %2, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @gt_rr(i32 %0) {
entry:
  %m = alloca i32, align 4
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  store i32 1, i32* %m, align 4
  %1 = load i32, i32* %n, align 4
  %2 = load i32, i32* %m, align 4
  %3 = icmp ugt i32 %1, %2
  br i1 %3, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @gte_ri(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  %1 = load i32, i32* %n, align 4
  %2 = icmp uge i32 %1, 1
  br i1 %2, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
}

define i32 @gte_rr(i32 %0) {
entry:
  %m = alloca i32, align 4
  %n = alloca i32, align 4
  store i32 %0, i32* %n, align 4
  store i32 1, i32* %m, align 4
  %1 = load i32, i32* %n, align 4
  %2 = load i32, i32* %m, align 4
  %3 = icmp uge i32 %1, %2
  br i1 %3, label %then, label %enif

then:                                             ; preds = %entry
  ret i32 2

enif:                                             ; preds = %entry
  ret i32 3
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
        assert_eq!(
            format!("{}", code.program),
            "main:
.LBL0_0:
  add r8 r8 4
  mstore [r8,-2] r8
  mov r1 1
  call eq_rr
  add r8 r8 -4
  end
eq_ri:
.LBL1_0:
  add r8 r8 1
  mov r0 r1
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  eq r0 r0 1
  cjmp r0 .LBL1_1
  jmp .LBL1_2
.LBL1_1:
  mov r0 2
  add r8 r8 -1
  ret
.LBL1_2:
  mov r0 3
  add r8 r8 -1
  ret
eq_rr:
.LBL2_0:
  add r8 r8 2
  mov r0 r1
  mstore [r8,-1] r0
  mov r0 1
  mstore [r8,-2] r0
  mload r0 [r8,-1]
  mload r1 [r8,-2]
  eq r0 r0 r1
  cjmp r0 .LBL2_1
  jmp .LBL2_2
.LBL2_1:
  mov r0 2
  add r8 r8 -2
  ret
.LBL2_2:
  mov r0 3
  add r8 r8 -2
  ret
neq_ri:
.LBL3_0:
  add r8 r8 1
  mov r0 r1
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  neq r0 r0 1
  cjmp r0 .LBL3_1
  jmp .LBL3_2
.LBL3_1:
  mov r0 2
  add r8 r8 -1
  ret
.LBL3_2:
  mov r0 3
  add r8 r8 -1
  ret
neq_rr:
.LBL4_0:
  add r8 r8 2
  mov r0 r1
  mstore [r8,-1] r0
  mov r0 1
  mstore [r8,-2] r0
  mload r0 [r8,-1]
  mload r1 [r8,-2]
  neq r0 r0 r1
  cjmp r0 .LBL4_1
  jmp .LBL4_2
.LBL4_1:
  mov r0 2
  add r8 r8 -2
  ret
.LBL4_2:
  mov r0 3
  add r8 r8 -2
  ret
lt_ri:
.LBL5_0:
  add r8 r8 1
  mov r0 r1
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  mov r1 1
  gte r1 r1 r0
  neq r0 r0 1
  and r1 r1 r0
  cjmp r1 .LBL5_1
  jmp .LBL5_2
.LBL5_1:
  mov r0 2
  add r8 r8 -1
  ret
.LBL5_2:
  mov r0 3
  add r8 r8 -1
  ret
lt_rr:
.LBL6_0:
  add r8 r8 2
  mov r0 r1
  mstore [r8,-1] r0
  mov r0 1
  mstore [r8,-2] r0
  mload r0 [r8,-1]
  mload r1 [r8,-2]
  gte r2 r1 r0
  neq r0 r0 r1
  and r2 r2 r0
  cjmp r2 .LBL6_1
  jmp .LBL6_2
.LBL6_1:
  mov r0 2
  add r8 r8 -2
  ret
.LBL6_2:
  mov r0 3
  add r8 r8 -2
  ret
lte_ri:
.LBL7_0:
  add r8 r8 1
  mov r0 r1
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  mov r7 1
  gte r0 r7 r0
  cjmp r0 .LBL7_1
  jmp .LBL7_2
.LBL7_1:
  mov r0 2
  add r8 r8 -1
  ret
.LBL7_2:
  mov r0 3
  add r8 r8 -1
  ret
lte_rr:
.LBL8_0:
  add r8 r8 2
  mov r0 r1
  mstore [r8,-1] r0
  mov r0 1
  mstore [r8,-2] r0
  mload r0 [r8,-1]
  mload r1 [r8,-2]
  gte r0 r1 r0
  cjmp r0 .LBL8_1
  jmp .LBL8_2
.LBL8_1:
  mov r0 2
  add r8 r8 -2
  ret
.LBL8_2:
  mov r0 3
  add r8 r8 -2
  ret
gt_ri:
.LBL9_0:
  add r8 r8 1
  mov r0 r1
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  gte r1 r0 1
  neq r0 r0 1
  and r1 r1 r0
  cjmp r1 .LBL9_1
  jmp .LBL9_2
.LBL9_1:
  mov r0 2
  add r8 r8 -1
  ret
.LBL9_2:
  mov r0 3
  add r8 r8 -1
  ret
gt_rr:
.LBL10_0:
  add r8 r8 2
  mov r0 r1
  mstore [r8,-1] r0
  mov r0 1
  mstore [r8,-2] r0
  mload r0 [r8,-1]
  mload r1 [r8,-2]
  gte r2 r0 r1
  neq r0 r0 r1
  and r2 r2 r0
  cjmp r2 .LBL10_1
  jmp .LBL10_2
.LBL10_1:
  mov r0 2
  add r8 r8 -2
  ret
.LBL10_2:
  mov r0 3
  add r8 r8 -2
  ret
gte_ri:
.LBL11_0:
  add r8 r8 1
  mov r0 r1
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  gte r0 r0 1
  cjmp r0 .LBL11_1
  jmp .LBL11_2
.LBL11_1:
  mov r0 2
  add r8 r8 -1
  ret
.LBL11_2:
  mov r0 3
  add r8 r8 -1
  ret
gte_rr:
.LBL12_0:
  add r8 r8 2
  mov r0 r1
  mstore [r8,-1] r0
  mov r0 1
  mstore [r8,-2] r0
  mload r0 [r8,-1]
  mload r1 [r8,-2]
  gte r0 r0 r1
  cjmp r0 .LBL12_1
  jmp .LBL12_2
.LBL12_1:
  mov r0 2
  add r8 r8 -2
  ret
.LBL12_2:
  mov r0 3
  add r8 r8 -2
  ret
"
        );
    }

    #[ignore]
    #[test]
    fn codegen_sqrt_test() {
        // LLVM Assembly
        let asm = r#"
        ; ModuleID = 'SqrtContract'
        source_filename = "examples/sqrt.ola"

        declare void @builtin_assert(i64, i64)

        declare void @builtin_range_check(i64)

        declare i64 @prophet_u32_sqrt(i64)

        define i64 @u32_sqrt(i64 %0) {
        entry:
          %1 = call i64 @prophet_u32_sqrt(i64 %0)
          call void @builtin_range_check(i64 %1)
          %2 = mul i64 %1, %1
          call void @builtin_assert(i64 %2, i64 %0)
          ret i64 %1
        }

        define void @main() {
        entry:
          %0 = call i64 @sqrt_test(i64 4)
          ret void
        }

        define i64 @sqrt_test(i64 %0) {
        entry:
          %b = alloca i64, align 8
          %n = alloca i64, align 8
          store i64 %0, i64* %n, align 8
          %1 = load i64, i64* %n, align 8
          %2 = call i64 @u32_sqrt(i64 %1)
          store i64 %2, i64* %b, align 8
          %3 = load i64, i64* %b, align 8
          ret i64 %3
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
        assert_eq!(
            format!("{}", code.program),
            "u32_sqrt:
.LBL3_0:
  mov r3 r1
  mov r1 r3
.PROPHET3_0:
  mov r0 psp
  mload r0 [r0,0]
  range r0
  mul r2 r0 r0
  assert r2 r3
  ret
main:
.LBL4_0:
  add r8 r8 4
  mstore [r8,-2] r8
  mov r1 4
  call sqrt_test
  add r8 r8 -4
  end
sqrt_test:
.LBL5_0:
  add r8 r8 6
  mstore [r8,-2] r8
  mov r0 r1
  mstore [r8,-3] r0
  mload r1 [r8,-3]
  call u32_sqrt
  mstore [r8,-4] r0
  mload r0 [r8,-4]
  add r8 r8 -6
  ret
"
        );
        assert_eq!(
            format!("{:#?}", code.prophets),
            r#"[
    Prophet {
        label: ".PROPHET3_0",
        code: "%{\n    entry() {\n        cid.y = sqrt(cid.x);\n    }\n%}",
        inputs: [
            "cid.x",
        ],
        outputs: [
            "cid.y",
        ],
    },
]"#
        );
    }

    #[ignore]
    #[test]
    fn codegen_sqrt_inst_test() {
        // LLVM Assembly
        let asm = r#"
; ModuleID = 'SqrtContract'
source_filename = "examples/sqrt.ola"

declare void @builtin_assert(i64, i64)

declare void @builtin_range_check(i64)

declare i64 @prophet_u32_sqrt(i64)

declare i64 @prophet_u32_div(i64, i64)

declare i64 @prophet_u32_mod(i64, i64)

define void @main() {
entry:
  %0 = call i64 @sqrt_test(i64 4)
  ret void
}

define i64 @sqrt_test(i64 %0) {
entry:
  %i = alloca i64, align 8
  %x = alloca i64, align 8
  %result = alloca i64, align 8
  %a = alloca i64, align 8
  store i64 %0, i64* %a, align 8
  store i64 0, i64* %result, align 8
  %1 = load i64, i64* %a, align 8
  %2 = icmp ugt i64 %1, 3
  br i1 %2, label %then, label %else

then:                                             ; preds = %entry
  %3 = load i64, i64* %a, align 8
  store i64 %3, i64* %result, align 4
  %4 = load i64, i64* %a, align 8
  %5 = call i64 @prophet_u32_mod(i64 %4, i64 2)
  call void @builtin_range_check(i64 %5)
  %6 = add i64 %5, 1
  %7 = sub i64 2, %6
  call void @builtin_range_check(i64 %7)
  %8 = call i64 @prophet_u32_div(i64 %4, i64 2)
  call void @builtin_range_check(i64 %8)
  %9 = mul i64 %8, 2
  %10 = add i64 %9, %5
  call void @builtin_assert(i64 %10, i64 %4)
  %11 = add i64 %8, 1
  call void @builtin_range_check(i64 %11)
  store i64 %11, i64* %x, align 8
  store i64 0, i64* %i, align 8
  br label %cond

else:                                             ; preds = %entry
  %12 = load i64, i64* %a, align 8
  %13 = icmp ne i64 %12, 0
  br i1 %13, label %then3, label %enif4

enif:                                             ; preds = %enif4, %endfor
  %14 = load i64, i64* %result, align 8
  ret i64 %14

cond:                                             ; preds = %next, %then
  %15 = load i64, i64* %i, align 8
  %16 = icmp ult i64 %15, 100
  br i1 %16, label %body, label %endfor

body:                                             ; preds = %cond
  %17 = load i64, i64* %x, align 8
  %18 = load i64, i64* %result, align 8
  %19 = icmp uge i64 %17, %18
  br i1 %19, label %then1, label %enif2

next:                                             ; preds = %enif2
  %20 = load i64, i64* %i, align 8
  %21 = add i64 %20, 1
  store i64 %21, i64* %i, align 8
  br label %cond

endfor:                                           ; preds = %then1, %cond
  br label %enif

then1:                                            ; preds = %body
  br label %endfor

enif2:                                            ; preds = %body
  %22 = load i64, i64* %x, align 8
  store i64 %22, i64* %result, align 4
  %23 = load i64, i64* %a, align 8
  %24 = load i64, i64* %x, align 8
  %25 = call i64 @prophet_u32_mod(i64 %23, i64 %24)
  call void @builtin_range_check(i64 %25)
  %26 = add i64 %25, 1
  %27 = sub i64 %24, %26
  call void @builtin_range_check(i64 %27)
  %28 = call i64 @prophet_u32_div(i64 %23, i64 %24)
  call void @builtin_range_check(i64 %28)
  %29 = mul i64 %28, %24
  %30 = add i64 %29, %25
  call void @builtin_assert(i64 %30, i64 %23)
  %31 = load i64, i64* %x, align 8
  %32 = add i64 %28, %31
  call void @builtin_range_check(i64 %32)
  %33 = call i64 @prophet_u32_mod(i64 %32, i64 2)
  call void @builtin_range_check(i64 %33)
  %34 = add i64 %33, 1
  %35 = sub i64 2, %34
  call void @builtin_range_check(i64 %35)
  %36 = call i64 @prophet_u32_div(i64 %32, i64 2)
  call void @builtin_range_check(i64 %36)
  %37 = mul i64 %36, 2
  %38 = add i64 %37, %33
  call void @builtin_assert(i64 %38, i64 %32)
  store i64 %36, i64* %x, align 4
  br label %next

then3:                                            ; preds = %else
  store i64 1, i64* %result, align 4
  br label %enif4

enif4:                                            ; preds = %then3, %else
  br label %enif
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
        assert_eq!(
            format!("{}", code.program),
            "main:
.LBL5_0:
  add r8 r8 4
  mstore [r8,-2] r8
  mov r1 4
  call sqrt_test
  add r8 r8 -4
  end
sqrt_test:
.LBL6_0:
  add r8 r8 17
  mov r0 r1
  mstore [r8,-14] r0
  mov r0 0
  mstore [r8,-15] r0
  mload r0 [r8,-14]
  gte r1 r0 3
  neq r0 r0 3
  and r1 r1 r0
  cjmp r1 .LBL6_1
  jmp .LBL6_2
.LBL6_1:
  mload r0 [r8,-14]
  mstore [r8,-15] r0
  mload r0 [r8,-14]
  mstore [r8,-11] r0
  mload r0 [r8,-11]
  mov r1 r0
  mov r2 2
.PROPHET6_0:
  mov r0 psp
  mload r0 [r0,0]
  mstore [r8,-10] r0
  mload r0 [r8,-10]
  range r0
  mov r0 2
  mload r1 [r8,-10]
  add r3 r1 1
  not r7 r3
  add r7 r7 1
  add r4 r0 r7
  range r4
  mload r0 [r8,-11]
  mov r1 r0
  mov r2 2
.PROPHET6_1:
  mov r0 psp
  mload r0 [r0,0]
  range r0
  mul r5 r0 2
  mload r1 [r8,-10]
  add r1 r5 r1
  mstore [r8,-13] r1
  mload r1 [r8,-13]
  mload r2 [r8,-11]
  assert r1 r2
  add r0 r0 1
  mstore [r8,-12] r0
  mload r0 [r8,-12]
  range r0
  mload r0 [r8,-12]
  mstore [r8,-16] r0
  mov r0 0
  mstore [r8,-17] r0
  jmp .LBL6_4
.LBL6_2:
  mload r0 [r8,-14]
  neq r0 r0 0
  cjmp r0 .LBL6_10
  jmp .LBL6_11
.LBL6_3:
  mload r0 [r8,-15]
  add r8 r8 -17
  ret
.LBL6_4:
  mload r0 [r8,-17]
  mov r1 100
  gte r1 r1 r0
  neq r0 r0 100
  and r1 r1 r0
  cjmp r1 .LBL6_5
  jmp .LBL6_7
.LBL6_5:
  mload r0 [r8,-16]
  mload r1 [r8,-15]
  gte r0 r0 r1
  cjmp r0 .LBL6_8
  jmp .LBL6_9
.LBL6_6:
  mload r1 [r8,-17]
  add r0 r1 1
  mstore [r8,-17] r0
  jmp .LBL6_4
.LBL6_7:
  jmp .LBL6_3
.LBL6_8:
  jmp .LBL6_7
.LBL6_9:
  mload r0 [r8,-16]
  mstore [r8,-15] r0
  mload r0 [r8,-14]
  mstore [r8,-3] r0
  mload r0 [r8,-16]
  mstore [r8,-2] r0
  mload r0 [r8,-3]
  mov r1 r0
  mload r0 [r8,-2]
  mov r2 r0
.PROPHET6_2:
  mov r0 psp
  mload r0 [r0,0]
  mstore [r8,-1] r0
  mload r0 [r8,-1]
  range r0
  mload r0 [r8,-1]
  add r3 r0 1
  not r7 r3
  add r7 r7 1
  mload r0 [r8,-2]
  add r4 r0 r7
  range r4
  mload r0 [r8,-3]
  mov r1 r0
  mload r0 [r8,-2]
  mov r2 r0
.PROPHET6_3:
  mov r0 psp
  mload r0 [r0,0]
  range r0
  mload r1 [r8,-2]
  mul r5 r0 r1
  mload r1 [r8,-1]
  add r1 r5 r1
  mstore [r8,-9] r1
  mload r1 [r8,-9]
  mload r2 [r8,-3]
  assert r1 r2
  mload r1 [r8,-16]
  add r0 r0 r1
  mstore [r8,-5] r0
  mload r0 [r8,-5]
  range r0
  mload r0 [r8,-5]
  mov r1 r0
  mov r2 2
.PROPHET6_4:
  mov r0 psp
  mload r0 [r0,0]
  mov r3 r0
  range r3
  mov r0 2
  add r1 r3 1
  mstore [r8,-8] r1
  mload r1 [r8,-8]
  not r7 r1
  add r7 r7 1
  add r0 r0 r7
  mstore [r8,-7] r0
  mload r0 [r8,-7]
  range r0
  mload r0 [r8,-5]
  mov r1 r0
  mov r2 2
.PROPHET6_5:
  mov r0 psp
  mload r0 [r0,0]
  range r0
  mul r1 r0 2
  mstore [r8,-6] r1
  mload r1 [r8,-6]
  add r1 r1 r3
  mstore [r8,-4] r1
  mload r1 [r8,-5]
  mload r2 [r8,-4]
  assert r2 r1
  mstore [r8,-16] r0
  jmp .LBL6_6
.LBL6_10:
  mov r0 1
  mstore [r8,-15] r0
  jmp .LBL6_11
.LBL6_11:
  jmp .LBL6_3
"
        );
    }
}
