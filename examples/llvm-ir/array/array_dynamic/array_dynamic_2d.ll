; ModuleID = 'DynamicTwoDimensionalArrayInMemory'
source_filename = "examples/source/array/array_dynamic/array_dynamic_2d.ola"

@heap_address = internal global i64 -4294967353

declare void @builtin_assert(i64)

declare void @builtin_range_check(i64)

declare i64 @prophet_u32_sqrt(i64)

declare i64 @prophet_u32_div(i64, i64)

declare i64 @prophet_u32_mod(i64, i64)

declare ptr @prophet_u32_array_sort(ptr, i64)

declare i64 @vector_new(i64)

declare void @get_context_data(i64, i64)

declare void @get_tape_data(i64, i64)

declare void @set_tape_data(i64, i64)

declare void @get_storage(ptr, ptr)

declare void @set_storage(ptr, ptr)

declare void @poseidon_hash(ptr, ptr, i64)

declare void @contract_call(ptr, i64)

declare void @prophet_printf(i64, i64)

define ptr @initialize(i64 %0, i64 %1) {
entry:
  %index_alloca11 = alloca i64, align 8
  %i = alloca i64, align 8
  %index_alloca = alloca i64, align 8
  %columns = alloca i64, align 8
  %rows = alloca i64, align 8
  store i64 %0, ptr %rows, align 4
  store i64 %1, ptr %columns, align 4
  %2 = load i64, ptr %rows, align 4
  %3 = call i64 @vector_new(i64 2)
  %heap_start = sub i64 %3, 2
  %heap_to_ptr = inttoptr i64 %heap_start to ptr
  store i64 1, ptr %heap_to_ptr, align 4
  %4 = ptrtoint ptr %heap_to_ptr to i64
  %5 = add i64 %4, 1
  %vector_data = inttoptr i64 %5 to ptr
  store i64 0, ptr %index_alloca, align 4
  br label %cond

cond:                                             ; preds = %body, %entry
  %index_value = load i64, ptr %index_alloca, align 4
  %loop_cond = icmp ult i64 %index_value, %2
  br i1 %loop_cond, label %body, label %done

body:                                             ; preds = %cond
  %index_access = getelementptr ptr, ptr %vector_data, i64 %index_value
  store ptr null, ptr %index_access, align 8
  %next_index = add i64 %index_value, 1
  store i64 %next_index, ptr %index_alloca, align 4
  br label %cond

done:                                             ; preds = %cond
  store i64 0, ptr %i, align 4
  br label %cond1

cond1:                                            ; preds = %next, %done
  %6 = load i64, ptr %i, align 4
  %7 = load i64, ptr %rows, align 4
  %8 = icmp ult i64 %6, %7
  br i1 %8, label %body2, label %endfor

body2:                                            ; preds = %cond1
  %9 = load i64, ptr %i, align 4
  %length = load i64, ptr %heap_to_ptr, align 4
  %10 = sub i64 %length, 1
  %11 = sub i64 %10, %9
  call void @builtin_range_check(i64 %11)
  %12 = ptrtoint ptr %heap_to_ptr to i64
  %13 = add i64 %12, 1
  %vector_data3 = inttoptr i64 %13 to ptr
  %index_access4 = getelementptr ptr, ptr %vector_data3, i64 %9
  %14 = load i64, ptr %columns, align 4
  %15 = call i64 @vector_new(i64 2)
  %heap_start5 = sub i64 %15, 2
  %heap_to_ptr6 = inttoptr i64 %heap_start5 to ptr
  store i64 1, ptr %heap_to_ptr6, align 4
  %16 = ptrtoint ptr %heap_to_ptr6 to i64
  %17 = add i64 %16, 1
  %vector_data7 = inttoptr i64 %17 to ptr
  store i64 0, ptr %index_alloca11, align 4
  br label %cond8

next:                                             ; preds = %done10
  %18 = load i64, ptr %i, align 4
  %19 = add i64 %18, 1
  store i64 %19, ptr %i, align 4
  br label %cond1

endfor:                                           ; preds = %cond1
  ret ptr %heap_to_ptr

cond8:                                            ; preds = %body9, %body2
  %index_value12 = load i64, ptr %index_alloca11, align 4
  %loop_cond13 = icmp ult i64 %index_value12, %14
  br i1 %loop_cond13, label %body9, label %done10

body9:                                            ; preds = %cond8
  %index_access14 = getelementptr i64, ptr %vector_data7, i64 %index_value12
  store i64 0, ptr %index_access14, align 4
  %next_index15 = add i64 %index_value12, 1
  store i64 %next_index15, ptr %index_alloca11, align 4
  br label %cond8

done10:                                           ; preds = %cond8
  store ptr %heap_to_ptr6, ptr %index_access4, align 8
  br label %next
}

define void @setElement(ptr %0, i64 %1, i64 %2, i64 %3) {
entry:
  %value = alloca i64, align 8
  %column = alloca i64, align 8
  %row = alloca i64, align 8
  %array = alloca ptr, align 8
  store ptr %0, ptr %array, align 8
  %4 = load ptr, ptr %array, align 8
  store i64 %1, ptr %row, align 4
  store i64 %2, ptr %column, align 4
  store i64 %3, ptr %value, align 4
  %5 = load i64, ptr %row, align 4
  %length = load i64, ptr %4, align 4
  %6 = sub i64 %length, 1
  %7 = sub i64 %6, %5
  call void @builtin_range_check(i64 %7)
  %8 = ptrtoint ptr %4 to i64
  %9 = add i64 %8, 1
  %vector_data = inttoptr i64 %9 to ptr
  %index_access = getelementptr ptr, ptr %vector_data, i64 %5
  %10 = load ptr, ptr %index_access, align 8
  %11 = load i64, ptr %column, align 4
  %length1 = load i64, ptr %10, align 4
  %12 = sub i64 %length1, 1
  %13 = sub i64 %12, %11
  call void @builtin_range_check(i64 %13)
  %14 = ptrtoint ptr %10 to i64
  %15 = add i64 %14, 1
  %vector_data2 = inttoptr i64 %15 to ptr
  %index_access3 = getelementptr i64, ptr %vector_data2, i64 %11
  %16 = load i64, ptr %value, align 4
  store i64 %16, ptr %index_access3, align 4
  ret void
}

define i64 @getElement(ptr %0, i64 %1, i64 %2) {
entry:
  %column = alloca i64, align 8
  %row = alloca i64, align 8
  %array = alloca ptr, align 8
  store ptr %0, ptr %array, align 8
  %3 = load ptr, ptr %array, align 8
  store i64 %1, ptr %row, align 4
  store i64 %2, ptr %column, align 4
  %4 = load i64, ptr %row, align 4
  %length = load i64, ptr %3, align 4
  %5 = sub i64 %length, 1
  %6 = sub i64 %5, %4
  call void @builtin_range_check(i64 %6)
  %7 = ptrtoint ptr %3 to i64
  %8 = add i64 %7, 1
  %vector_data = inttoptr i64 %8 to ptr
  %index_access = getelementptr ptr, ptr %vector_data, i64 %4
  %9 = load ptr, ptr %index_access, align 8
  %10 = load i64, ptr %column, align 4
  %length1 = load i64, ptr %9, align 4
  %11 = sub i64 %length1, 1
  %12 = sub i64 %11, %10
  call void @builtin_range_check(i64 %12)
  %13 = ptrtoint ptr %9 to i64
  %14 = add i64 %13, 1
  %vector_data2 = inttoptr i64 %14 to ptr
  %index_access3 = getelementptr i64, ptr %vector_data2, i64 %10
  %15 = load i64, ptr %index_access3, align 4
  ret i64 %15
}

define void @function_dispatch(i64 %0, i64 %1, ptr %2) {
entry:
  %size_var100 = alloca i64, align 8
  %size_var62 = alloca i64, align 8
  %size_var = alloca i64, align 8
  %input_alloca = alloca ptr, align 8
  store ptr %2, ptr %input_alloca, align 8
  %input = load ptr, ptr %input_alloca, align 8
  switch i64 %0, label %missing_function [
    i64 4288712523, label %func_0_dispatch
    i64 1779028247, label %func_1_dispatch
    i64 289838681, label %func_2_dispatch
  ]

missing_function:                                 ; preds = %entry
  unreachable

func_0_dispatch:                                  ; preds = %entry
  %input_start = ptrtoint ptr %input to i64
  %3 = inttoptr i64 %input_start to ptr
  %decode_value = load i64, ptr %3, align 4
  %4 = add i64 %input_start, 1
  %5 = inttoptr i64 %4 to ptr
  %decode_value1 = load i64, ptr %5, align 4
  %6 = call ptr @initialize(i64 %decode_value, i64 %decode_value1)
  store i64 0, ptr %size_var, align 4
  %length = load i64, ptr %6, align 4
  %7 = load i64, ptr %size_var, align 4
  %8 = add i64 %7, %length
  store i64 %8, ptr %size_var, align 4
  %index_ptr = alloca i64, align 8
  store i64 0, ptr %index_ptr, align 4
  %index = load i64, ptr %index_ptr, align 4
  br label %cond

cond:                                             ; preds = %next, %func_0_dispatch
  %length2 = load i64, ptr %6, align 4
  %9 = icmp ult i64 %index, %length2
  br i1 %9, label %body, label %end_for

next:                                             ; preds = %end_for10
  %index24 = load i64, ptr %index_ptr, align 4
  %10 = add i64 %index24, 1
  store i64 %10, ptr %index_ptr, align 4
  br label %cond

body:                                             ; preds = %cond
  %length3 = load i64, ptr %6, align 4
  %11 = sub i64 %length3, 1
  %12 = sub i64 %11, %index
  call void @builtin_range_check(i64 %12)
  %13 = ptrtoint ptr %6 to i64
  %14 = add i64 %13, 1
  %vector_data = inttoptr i64 %14 to ptr
  %index_access = getelementptr ptr, ptr %vector_data, i64 %index
  %arr = load ptr, ptr %index_access, align 8
  %length4 = load i64, ptr %arr, align 4
  %15 = load i64, ptr %size_var, align 4
  %16 = add i64 %15, %length4
  store i64 %16, ptr %size_var, align 4
  %index_ptr5 = alloca i64, align 8
  store i64 0, ptr %index_ptr5, align 4
  %index6 = load i64, ptr %index_ptr5, align 4
  br label %cond7

end_for:                                          ; preds = %cond
  %17 = load i64, ptr %size_var, align 4
  %heap_size = add i64 %17, 1
  %18 = call i64 @vector_new(i64 %heap_size)
  %heap_start = sub i64 %18, %heap_size
  %heap_to_ptr = inttoptr i64 %heap_start to ptr
  %offset_var_no = alloca i64, align 8
  store i64 0, ptr %offset_var_no, align 4
  %length25 = load i64, ptr %6, align 4
  %19 = load i64, ptr %offset_var_no, align 4
  %encode_value_ptr = getelementptr i64, ptr %heap_to_ptr, i64 %19
  store i64 %length25, ptr %encode_value_ptr, align 4
  %20 = add i64 %19, 1
  store i64 %20, ptr %offset_var_no, align 4
  %index_ptr26 = alloca i64, align 8
  store i64 0, ptr %index_ptr26, align 4
  %index27 = load i64, ptr %index_ptr26, align 4
  br label %cond28

cond7:                                            ; preds = %next8, %body
  %length11 = load i64, ptr %6, align 4
  %21 = sub i64 %length11, 1
  %22 = sub i64 %21, %index
  call void @builtin_range_check(i64 %22)
  %23 = ptrtoint ptr %6 to i64
  %24 = add i64 %23, 1
  %vector_data12 = inttoptr i64 %24 to ptr
  %index_access13 = getelementptr ptr, ptr %vector_data12, i64 %index
  %arr14 = load ptr, ptr %index_access13, align 8
  %length15 = load i64, ptr %arr14, align 4
  %25 = icmp ult i64 %index6, %length15
  br i1 %25, label %body9, label %end_for10

next8:                                            ; preds = %body9
  %index23 = load i64, ptr %index_ptr5, align 4
  %26 = add i64 %index23, 1
  store i64 %26, ptr %index_ptr5, align 4
  br label %cond7

body9:                                            ; preds = %cond7
  %length16 = load i64, ptr %6, align 4
  %27 = sub i64 %length16, 1
  %28 = sub i64 %27, %index
  call void @builtin_range_check(i64 %28)
  %29 = ptrtoint ptr %6 to i64
  %30 = add i64 %29, 1
  %vector_data17 = inttoptr i64 %30 to ptr
  %index_access18 = getelementptr ptr, ptr %vector_data17, i64 %index
  %arr19 = load ptr, ptr %index_access18, align 8
  %length20 = load i64, ptr %arr19, align 4
  %31 = sub i64 %length20, 1
  %32 = sub i64 %31, %index6
  call void @builtin_range_check(i64 %32)
  %33 = ptrtoint ptr %arr19 to i64
  %34 = add i64 %33, 1
  %vector_data21 = inttoptr i64 %34 to ptr
  %index_access22 = getelementptr i64, ptr %vector_data21, i64 %index6
  %35 = load i64, ptr %size_var, align 4
  %36 = add i64 %35, 1
  store i64 %36, ptr %size_var, align 4
  br label %next8

end_for10:                                        ; preds = %cond7
  br label %next

cond28:                                           ; preds = %next29, %end_for
  %length32 = load i64, ptr %6, align 4
  %37 = icmp ult i64 %index27, %length32
  br i1 %37, label %body30, label %end_for31

next29:                                           ; preds = %end_for44
  %index59 = load i64, ptr %index_ptr26, align 4
  %38 = add i64 %index59, 1
  store i64 %38, ptr %index_ptr26, align 4
  br label %cond28

body30:                                           ; preds = %cond28
  %length33 = load i64, ptr %6, align 4
  %39 = sub i64 %length33, 1
  %40 = sub i64 %39, %index27
  call void @builtin_range_check(i64 %40)
  %41 = ptrtoint ptr %6 to i64
  %42 = add i64 %41, 1
  %vector_data34 = inttoptr i64 %42 to ptr
  %index_access35 = getelementptr ptr, ptr %vector_data34, i64 %index27
  %arr36 = load ptr, ptr %index_access35, align 8
  %length37 = load i64, ptr %arr36, align 4
  %43 = load i64, ptr %offset_var_no, align 4
  %encode_value_ptr38 = getelementptr i64, ptr %heap_to_ptr, i64 %43
  store i64 %length37, ptr %encode_value_ptr38, align 4
  %44 = add i64 %43, 1
  store i64 %44, ptr %offset_var_no, align 4
  %index_ptr39 = alloca i64, align 8
  store i64 0, ptr %index_ptr39, align 4
  %index40 = load i64, ptr %index_ptr39, align 4
  br label %cond41

end_for31:                                        ; preds = %cond28
  %45 = load i64, ptr %offset_var_no, align 4
  %encode_value_ptr60 = getelementptr i64, ptr %heap_to_ptr, i64 %45
  store i64 %17, ptr %encode_value_ptr60, align 4
  call void @set_tape_data(i64 %heap_start, i64 %heap_size)
  ret void

cond41:                                           ; preds = %next42, %body30
  %length45 = load i64, ptr %arr36, align 4
  %46 = sub i64 %length45, 1
  %47 = sub i64 %46, %index27
  call void @builtin_range_check(i64 %47)
  %48 = ptrtoint ptr %arr36 to i64
  %49 = add i64 %48, 1
  %vector_data46 = inttoptr i64 %49 to ptr
  %index_access47 = getelementptr i64, ptr %vector_data46, i64 %index27
  %arr48 = load ptr, ptr %index_access47, align 8
  %length49 = load i64, ptr %arr48, align 4
  %50 = icmp ult i64 %index40, %length49
  br i1 %50, label %body43, label %end_for44

next42:                                           ; preds = %body43
  %index58 = load i64, ptr %index_ptr39, align 4
  %51 = add i64 %index58, 1
  store i64 %51, ptr %index_ptr39, align 4
  br label %cond41

body43:                                           ; preds = %cond41
  %length50 = load i64, ptr %arr36, align 4
  %52 = sub i64 %length50, 1
  %53 = sub i64 %52, %index27
  call void @builtin_range_check(i64 %53)
  %54 = ptrtoint ptr %arr36 to i64
  %55 = add i64 %54, 1
  %vector_data51 = inttoptr i64 %55 to ptr
  %index_access52 = getelementptr i64, ptr %vector_data51, i64 %index27
  %arr53 = load ptr, ptr %index_access52, align 8
  %length54 = load i64, ptr %arr53, align 4
  %56 = sub i64 %length54, 1
  %57 = sub i64 %56, %index40
  call void @builtin_range_check(i64 %57)
  %58 = ptrtoint ptr %arr53 to i64
  %59 = add i64 %58, 1
  %vector_data55 = inttoptr i64 %59 to ptr
  %index_access56 = getelementptr i64, ptr %vector_data55, i64 %index40
  %60 = load i64, ptr %offset_var_no, align 4
  %encode_value_ptr57 = getelementptr i64, ptr %heap_to_ptr, i64 %60
  store ptr %index_access56, ptr %encode_value_ptr57, align 8
  %61 = add i64 %60, 1
  store i64 %61, ptr %offset_var_no, align 4
  br label %next42

end_for44:                                        ; preds = %cond41
  br label %next29

func_1_dispatch:                                  ; preds = %entry
  %input_start61 = ptrtoint ptr %input to i64
  %62 = inttoptr i64 %input_start61 to ptr
  store i64 0, ptr %size_var62, align 4
  %length63 = load i64, ptr %62, align 4
  %63 = load i64, ptr %size_var62, align 4
  %64 = add i64 %63, %length63
  store i64 %64, ptr %size_var62, align 4
  %index_ptr64 = alloca i64, align 8
  store i64 0, ptr %index_ptr64, align 4
  %index65 = load i64, ptr %index_ptr64, align 4
  br label %cond66

cond66:                                           ; preds = %next67, %func_1_dispatch
  %length70 = load i64, ptr %62, align 4
  %65 = icmp ult i64 %index65, %length70
  br i1 %65, label %body68, label %end_for69

next67:                                           ; preds = %end_for81
  %index95 = load i64, ptr %index_ptr64, align 4
  %66 = add i64 %index95, 1
  store i64 %66, ptr %index_ptr64, align 4
  br label %cond66

body68:                                           ; preds = %cond66
  %length71 = load i64, ptr %62, align 4
  %67 = sub i64 %length71, 1
  %68 = sub i64 %67, %index65
  call void @builtin_range_check(i64 %68)
  %69 = ptrtoint ptr %62 to i64
  %70 = add i64 %69, 1
  %vector_data72 = inttoptr i64 %70 to ptr
  %index_access73 = getelementptr ptr, ptr %vector_data72, i64 %index65
  %arr74 = load ptr, ptr %index_access73, align 8
  %length75 = load i64, ptr %arr74, align 4
  %71 = load i64, ptr %size_var62, align 4
  %72 = add i64 %71, %length75
  store i64 %72, ptr %size_var62, align 4
  %index_ptr76 = alloca i64, align 8
  store i64 0, ptr %index_ptr76, align 4
  %index77 = load i64, ptr %index_ptr76, align 4
  br label %cond78

end_for69:                                        ; preds = %cond66
  %73 = load i64, ptr %size_var62, align 4
  %74 = add i64 %input_start61, %73
  %75 = inttoptr i64 %74 to ptr
  %decode_value96 = load i64, ptr %75, align 4
  %76 = add i64 %74, 1
  %77 = inttoptr i64 %76 to ptr
  %decode_value97 = load i64, ptr %77, align 4
  %78 = add i64 %76, 1
  %79 = inttoptr i64 %78 to ptr
  %decode_value98 = load i64, ptr %79, align 4
  call void @setElement(ptr %62, i64 %decode_value96, i64 %decode_value97, i64 %decode_value98)
  ret void

cond78:                                           ; preds = %next79, %body68
  %length82 = load i64, ptr %62, align 4
  %80 = sub i64 %length82, 1
  %81 = sub i64 %80, %index65
  call void @builtin_range_check(i64 %81)
  %82 = ptrtoint ptr %62 to i64
  %83 = add i64 %82, 1
  %vector_data83 = inttoptr i64 %83 to ptr
  %index_access84 = getelementptr ptr, ptr %vector_data83, i64 %index65
  %arr85 = load ptr, ptr %index_access84, align 8
  %length86 = load i64, ptr %arr85, align 4
  %84 = icmp ult i64 %index77, %length86
  br i1 %84, label %body80, label %end_for81

next79:                                           ; preds = %body80
  %index94 = load i64, ptr %index_ptr76, align 4
  %85 = add i64 %index94, 1
  store i64 %85, ptr %index_ptr76, align 4
  br label %cond78

body80:                                           ; preds = %cond78
  %length87 = load i64, ptr %62, align 4
  %86 = sub i64 %length87, 1
  %87 = sub i64 %86, %index65
  call void @builtin_range_check(i64 %87)
  %88 = ptrtoint ptr %62 to i64
  %89 = add i64 %88, 1
  %vector_data88 = inttoptr i64 %89 to ptr
  %index_access89 = getelementptr ptr, ptr %vector_data88, i64 %index65
  %arr90 = load ptr, ptr %index_access89, align 8
  %length91 = load i64, ptr %arr90, align 4
  %90 = sub i64 %length91, 1
  %91 = sub i64 %90, %index77
  call void @builtin_range_check(i64 %91)
  %92 = ptrtoint ptr %arr90 to i64
  %93 = add i64 %92, 1
  %vector_data92 = inttoptr i64 %93 to ptr
  %index_access93 = getelementptr i64, ptr %vector_data92, i64 %index77
  %94 = load i64, ptr %size_var62, align 4
  %95 = add i64 %94, 1
  store i64 %95, ptr %size_var62, align 4
  br label %next79

end_for81:                                        ; preds = %cond78
  br label %next67

func_2_dispatch:                                  ; preds = %entry
  %input_start99 = ptrtoint ptr %input to i64
  %96 = inttoptr i64 %input_start99 to ptr
  store i64 0, ptr %size_var100, align 4
  %length101 = load i64, ptr %96, align 4
  %97 = load i64, ptr %size_var100, align 4
  %98 = add i64 %97, %length101
  store i64 %98, ptr %size_var100, align 4
  %index_ptr102 = alloca i64, align 8
  store i64 0, ptr %index_ptr102, align 4
  %index103 = load i64, ptr %index_ptr102, align 4
  br label %cond104

cond104:                                          ; preds = %next105, %func_2_dispatch
  %length108 = load i64, ptr %96, align 4
  %99 = icmp ult i64 %index103, %length108
  br i1 %99, label %body106, label %end_for107

next105:                                          ; preds = %end_for119
  %index133 = load i64, ptr %index_ptr102, align 4
  %100 = add i64 %index133, 1
  store i64 %100, ptr %index_ptr102, align 4
  br label %cond104

body106:                                          ; preds = %cond104
  %length109 = load i64, ptr %96, align 4
  %101 = sub i64 %length109, 1
  %102 = sub i64 %101, %index103
  call void @builtin_range_check(i64 %102)
  %103 = ptrtoint ptr %96 to i64
  %104 = add i64 %103, 1
  %vector_data110 = inttoptr i64 %104 to ptr
  %index_access111 = getelementptr ptr, ptr %vector_data110, i64 %index103
  %arr112 = load ptr, ptr %index_access111, align 8
  %length113 = load i64, ptr %arr112, align 4
  %105 = load i64, ptr %size_var100, align 4
  %106 = add i64 %105, %length113
  store i64 %106, ptr %size_var100, align 4
  %index_ptr114 = alloca i64, align 8
  store i64 0, ptr %index_ptr114, align 4
  %index115 = load i64, ptr %index_ptr114, align 4
  br label %cond116

end_for107:                                       ; preds = %cond104
  %107 = load i64, ptr %size_var100, align 4
  %108 = add i64 %input_start99, %107
  %109 = inttoptr i64 %108 to ptr
  %decode_value134 = load i64, ptr %109, align 4
  %110 = add i64 %108, 1
  %111 = inttoptr i64 %110 to ptr
  %decode_value135 = load i64, ptr %111, align 4
  %112 = call i64 @getElement(ptr %96, i64 %decode_value134, i64 %decode_value135)
  %113 = call i64 @vector_new(i64 2)
  %heap_start136 = sub i64 %113, 2
  %heap_to_ptr137 = inttoptr i64 %heap_start136 to ptr
  %encode_value_ptr138 = getelementptr i64, ptr %heap_to_ptr137, i64 0
  store i64 %112, ptr %encode_value_ptr138, align 4
  %encode_value_ptr139 = getelementptr i64, ptr %heap_to_ptr137, i64 1
  store i64 1, ptr %encode_value_ptr139, align 4
  call void @set_tape_data(i64 %heap_start136, i64 2)
  ret void

cond116:                                          ; preds = %next117, %body106
  %length120 = load i64, ptr %96, align 4
  %114 = sub i64 %length120, 1
  %115 = sub i64 %114, %index103
  call void @builtin_range_check(i64 %115)
  %116 = ptrtoint ptr %96 to i64
  %117 = add i64 %116, 1
  %vector_data121 = inttoptr i64 %117 to ptr
  %index_access122 = getelementptr ptr, ptr %vector_data121, i64 %index103
  %arr123 = load ptr, ptr %index_access122, align 8
  %length124 = load i64, ptr %arr123, align 4
  %118 = icmp ult i64 %index115, %length124
  br i1 %118, label %body118, label %end_for119

next117:                                          ; preds = %body118
  %index132 = load i64, ptr %index_ptr114, align 4
  %119 = add i64 %index132, 1
  store i64 %119, ptr %index_ptr114, align 4
  br label %cond116

body118:                                          ; preds = %cond116
  %length125 = load i64, ptr %96, align 4
  %120 = sub i64 %length125, 1
  %121 = sub i64 %120, %index103
  call void @builtin_range_check(i64 %121)
  %122 = ptrtoint ptr %96 to i64
  %123 = add i64 %122, 1
  %vector_data126 = inttoptr i64 %123 to ptr
  %index_access127 = getelementptr ptr, ptr %vector_data126, i64 %index103
  %arr128 = load ptr, ptr %index_access127, align 8
  %length129 = load i64, ptr %arr128, align 4
  %124 = sub i64 %length129, 1
  %125 = sub i64 %124, %index115
  call void @builtin_range_check(i64 %125)
  %126 = ptrtoint ptr %arr128 to i64
  %127 = add i64 %126, 1
  %vector_data130 = inttoptr i64 %127 to ptr
  %index_access131 = getelementptr i64, ptr %vector_data130, i64 %index115
  %128 = load i64, ptr %size_var100, align 4
  %129 = add i64 %128, 1
  store i64 %129, ptr %size_var100, align 4
  br label %next117

end_for119:                                       ; preds = %cond116
  br label %next105
}

define void @main() {
entry:
  %0 = call i64 @vector_new(i64 13)
  %heap_start = sub i64 %0, 13
  %heap_to_ptr = inttoptr i64 %heap_start to ptr
  call void @get_tape_data(i64 %heap_start, i64 13)
  %function_selector = load i64, ptr %heap_to_ptr, align 4
  %1 = call i64 @vector_new(i64 14)
  %heap_start1 = sub i64 %1, 14
  %heap_to_ptr2 = inttoptr i64 %heap_start1 to ptr
  call void @get_tape_data(i64 %heap_start1, i64 14)
  %input_length = load i64, ptr %heap_to_ptr2, align 4
  %2 = add i64 %input_length, 14
  %3 = call i64 @vector_new(i64 %2)
  %heap_start3 = sub i64 %3, %2
  %heap_to_ptr4 = inttoptr i64 %heap_start3 to ptr
  call void @get_tape_data(i64 %heap_start3, i64 %2)
  call void @function_dispatch(i64 %function_selector, i64 %input_length, ptr %heap_to_ptr4)
  ret void
}