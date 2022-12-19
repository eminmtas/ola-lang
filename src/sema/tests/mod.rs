// SPDX-License-Identifier: Apache-2.0

#![cfg(test)]
use crate::sema::ast::{Expression, Parameter, Statement, Type};
use crate::sema::diagnostics::Diagnostics;
use crate::{parse_and_resolve, sema::ast, FileResolver};
use ola_parser::program::Loc;
use std::ffi::OsStr;

pub(crate) fn parse(src: &'static str) -> ast::Namespace {
    let mut cache = FileResolver::new();
    cache.set_file_contents("test.ola", src.to_string());

    let ns = parse_and_resolve(OsStr::new("test.ola"), &mut cache);
    ns
}

#[test]
fn test_statement_reachable() {
    let loc = Loc::File(0, 1, 2);
    let test_cases: Vec<(Statement, bool)> = vec![
        (Statement::Underscore(loc), true),
        (
            Statement::VariableDecl(
                loc,
                0,
                Parameter {
                    loc,
                    id: None,
                    ty: Type::Bool,
                    ty_loc: None,
                    recursive: false,
                },
                None,
            ),
            true,
        ),
        (Statement::Continue(loc), false),
        (Statement::Break(loc), false),
        (Statement::Return(loc, None), false),
        (
            Statement::If(
                loc,
                false,
                Expression::BoolLiteral(loc, false),
                vec![],
                vec![],
            ),
            false,
        ),
        (
            Statement::Expression(loc, true, Expression::BoolLiteral(loc, false)),
            true,
        ),
        (
            Statement::For {
                loc,
                reachable: false,
                init: vec![],
                cond: None,
                next: vec![],
                body: vec![],
            },
            false,
        ),
    ];

    for (test_case, expected) in test_cases {
        assert_eq!(test_case.reachable(), expected);
    }
}

#[test]
fn constant_overflow_checks() {
    let file = r#"
    contract test_contract {
        fn test_params(u32 usesa, u32 sesa) -> (u32) {

            return usesa + sesa;
        }

         fn test_add(u32 input) -> (u32) {
            // value 4294967297 does not fit into type u32.
            u32 add_ovf = 4294967295 + 2;

            // negative value -1 does not fit into type u32. Cannot implicitly convert signed literal to unsigned type.
            u32 negative = 3 - 4;

            // value 4294967295 does not fit into type u32.
            u32 mixed = 4294967295 + 2 + input;

            // negative value -1 does not fit into type u32. Cannot implicitly convert signed literal to unsigned type.
            return 1 - 2;
        }

        fn test_mul(u32 input)  {
            // value 4294967296 does not fit into type u32.
            u32 mul_ovf_u32 = 2147483647 * 2 + 2;
            // value 9223372036854775808 does not fit into type u64.
            u64 mul_ovf_u64 = 4611686018427387903 * 2 + 2;
        }

        fn test_shift(u32 input) {
            // value 4294967296 does not fit into type u32.
            u32 mul_ovf = 2147483648 << 2;

            // value 4294967296 does not fit into type u.
            u32 mul_ovf_u32 = 2147483647 * 2 + 2;
            // value 9223372036854775808 does not fit into type u64.
            u64 mul_ovf_u64 = 4611686018427387903 * 2 + 2;

            // value 128 does not fit into type int8.
            // warning: left shift by 7 may overflow the final result
            int8 mixed = (1 << 7) + input;
        }
        //
        // function test_call() public {
        //     // negative value -1 does not fit into type uint8. Cannot implicitly convert signed literal to unsigned type.
        //     // value 129 does not fit into type int8.
        //     test_params(1 - 2, 127 + 2);
        //
        //     // negative value -1 does not fit into type uint8. Cannot implicitly convert signed literal to unsigned type.
        //     // value 129 does not fit into type int8.
        //     test_params({usesa: 1 - 2, sesa: 127 + 2});
        // }
        //
        // function test_builtin (bytes input) public{
        //
        //     // value 4294967296 does not fit into type uint32.
        //     int16 sesa = input.readInt16LE(4294967296);
        // }
        //
        // function test_for_loop () public {
        //     for (int8 i = 125 + 5; i < 300 ; i++) {
        //     }
        // }
        //
        // function composite(int8 a, bytes input) public{
        //
        //     uint8 sesa = 500- 400 + test_params(100+200, 0) + (200+101) + input.readUint8(4294967296);
        //     int8 seas = (120 + 120) + a + (120 + 125);
        //
        //     // no diagnostic
        //     uint8 b = 255 - 255/5 ;
        //
        //     // value 260 does not fit into type uint8.
        //     uint8 shift_r = (120 >> 2) + 230;
        //
        //     // value 261 does not fit into type uint8.
        //     uint8 mod_test = 254 + (500%17);
        //
        //     // value 269 does not fit into type uint8.
        //     uint8 bb = 320 - (255/5) ;
        //
        //     // left shift by 7 may overflow the final result
        //     uint8 shift_warning = (1 << 9) - 300;
        //
        //     int8 bitwise_or = (250 | 5) - 150;
        //
        //     // value 155 does not fit into type int8.
        //     int8 bitwise_or_ovf = (250 | 5) - 100;
        //
        //     uint8 bitwise_and = 1000 & 5 ;
        //
        //     // value 262 does not fit into type uint8.
        //     uint8 bitwise_and_ovf = (1000 & 255) + 30 ;
        //
        //     uint8 bitwise_xor = 1000 ^ 256;
        //
        //     // divide by zero
        //     uint8 div_zero= 3 / (1-1);
        //
        //     // divide by zero
        //     uint8 div_zeroo = (300-50) % 0;
        //
        //     // shift by negative number not allowed.
        //     uint8 shift_left_neg = 120 << -1;
        //     uint8 shift_right_neg = 120 >> -1;
        //
        //     // power by -1 is not allowed.
        //     uint8 pow = 12 ** -1;
        //
        //     // large shift not allowed
        //     int x = 1 >> 14676683207225698178084221555689649093015162623576402558976;
        //
        // }
    }
    
        "#;
    let ns = parse(file);
    let errors = ns.diagnostics.errors();
    let warnings = ns.diagnostics.warnings();

    assert_eq!(errors[0].message, "value 4294967297 does not fit into type u32.");
    assert_eq!(errors[1].message, "negative value -1 does not fit into type u32. Cannot implicitly convert signed literal to unsigned type.");
    assert_eq!(errors[2].message, "value 4294967297 does not fit into type u32.");
    assert_eq!(errors[3].message, "negative value -1 does not fit into type u32. Cannot implicitly convert signed literal to unsigned type.");
    assert_eq!(errors[4].message, "value 4294967296 does not fit into type u32.");
    assert_eq!(errors[5].message, "value 9223372036854775808 does not fit into type u64.");
    assert_eq!(errors[6].message, "value 128 does not fit into type int8.");
    assert_eq!(errors[7].message, "value 128 does not fit into type int8.");
    assert_eq!(errors[8].message, "negative value -1 does not fit into type uint8. Cannot implicitly convert signed literal to unsigned type.");
    assert_eq!(errors[9].message, "value 129 does not fit into type int8.");
    assert_eq!(errors[10].message, "negative value -1 does not fit into type uint8. Cannot implicitly convert signed literal to unsigned type.");
    assert_eq!(errors[11].message, "value 129 does not fit into type int8.");
    assert_eq!(
        errors[12].message,
        "value 4294967296 does not fit into type uint32."
    );
    assert_eq!(errors[13].message, "value 130 does not fit into type int8.");
    assert_eq!(
        errors[14].message,
        "value 300 does not fit into type uint8."
    );
    assert_eq!(
        errors[15].message,
        "value 301 does not fit into type uint8."
    );
    assert_eq!(
        errors[16].message,
        "value 4294967296 does not fit into type uint32."
    );
    assert_eq!(errors[17].message, "value 240 does not fit into type int8.");
    assert_eq!(errors[18].message, "value 245 does not fit into type int8.");
    assert_eq!(
        errors[19].message,
        "value 260 does not fit into type uint8."
    );
    assert_eq!(
        errors[20].message,
        "value 261 does not fit into type uint8."
    );
    assert_eq!(
        errors[21].message,
        "value 269 does not fit into type uint8."
    );
    assert_eq!(errors[22].message, "value 155 does not fit into type int8.");
    assert_eq!(
        errors[23].message,
        "value 262 does not fit into type uint8."
    );

    assert_eq!(
        errors[24].message,
        "value 744 does not fit into type uint8."
    );
    assert_eq!(errors[25].message, "divide by zero");
    assert_eq!(errors[26].message, "divide by zero");
    assert_eq!(errors[27].message, "left shift by -1 is not possible");
    assert_eq!(errors[28].message, "right shift by -1 is not possible");
    assert_eq!(errors[29].message, "power by -1 is not possible");
    assert_eq!(errors[30].message, "right shift by 14676683207225698178084221555689649093015162623576402558976 is not possible");

    assert_eq!(errors.len(), 31);

    assert_eq!(
        warnings[0].message,
        "left shift by 7 may overflow the final result"
    );
    assert_eq!(
        warnings[1].message,
        "left shift by 7 may overflow the final result"
    );
    assert_eq!(
        warnings[2].message,
        "left shift by 9 may overflow the final result"
    );
    assert_eq!(warnings.len(), 3);
}

#[test]
fn test_types() {
    let file = r#"
    contract test_contract {
        function test_types32(bytes input) public {
            // value 2147483648 does not fit into type int32.
            int32 add_ovf = 2147483647 + 1;
    
            // value 2147483648 does not fit into type int32.
            int32 add_normal = 2147483647 + 0;
    
            // value 2147483648 does not fit into type int32.
            int32 mixed = 2147483647 + 1 + input.readInt32LE(2);
        }
    
        function test_types64(bytes input) public {
            // value 9223372036854775808 does not fit into type int64.
            int64 add_ovf = 9223372036854775807 + 1;
    
            int64 add_normal = 9223372036854775807;
    
            // value 9223372036854775808 does not fit into type int64.
            int64 mixed = 9223372036854775807 + 1 + input.readInt64LE(2);
    
            // value 18446744073709551616 does not fit into type uint64.
            uint64 pow_ovf = 2**64;
    
            uint64 normal_pow = (2**64) - 1;
        }
    
        function test_types_128_256(bytes input) public {
            while (true) {
                // value 340282366920938463463374607431768211456 does not fit into type uint64.
                uint128 ovf = 2**128;
                uint128 normal = 2**128 - 1;
            }
            uint128[] arr;
            // negative value -1 does not fit into type uint32. Cannot implicitly convert signed literal to unsigned type.
            // value 340282366920938463463374607431768211456 does not fit into type uint128.
            uint128 access = arr[1 - 2] + 1 + (2**128);
            // value 3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000 does not fit into type uint256.
            uint256 num = 3e255;
    
            uint256 num_2 = 115792089237316195423570985008687907853269984665640564039457584007913129639935 *
                    4;
        }
    
        function foo() public {
            uint16 x = 0;
            x += 450000;
    
            for (uint16 i = 0; i < (2**32); i += 65546) {}
    
            uint8 y = 0;
            y *= 120 + 250;
            y -= 500;
            y /= 300 + 200 - 200 + y;
        }
    }
    
        "#;
    let ns = parse(file);
    let errors = ns.diagnostics.errors();

    assert_eq!(
        errors[0].message,
        "value 2147483648 does not fit into type int32."
    );
    assert_eq!(
        errors[1].message,
        "value 2147483648 does not fit into type int32."
    );
    assert_eq!(
        errors[2].message,
        "value 9223372036854775808 does not fit into type int64."
    );
    assert_eq!(
        errors[3].message,
        "value 9223372036854775808 does not fit into type int64."
    );
    assert_eq!(
        errors[4].message,
        "value 18446744073709551616 does not fit into type uint64."
    );
    assert_eq!(
        errors[5].message,
        "value 340282366920938463463374607431768211456 does not fit into type uint128."
    );
    assert_eq!(errors[6].message, "negative value -1 does not fit into type uint32. Cannot implicitly convert signed literal to unsigned type.");
    assert_eq!(
        errors[7].message,
        "value 340282366920938463463374607431768211456 does not fit into type uint128."
    );

    assert_eq!(errors[8].message, "value 3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000 does not fit into type uint256.");

    assert_eq!(errors[9].message, "value 463168356949264781694283940034751631413079938662562256157830336031652518559740 does not fit into type uint256.");
    assert_eq!(
        errors[10].message,
        "value 450000 does not fit into type uint16."
    );
    assert_eq!(
        errors[11].message,
        "value 65546 does not fit into type uint16."
    );
    assert_eq!(
        errors[12].message,
        "value 370 does not fit into type uint8."
    );
    assert_eq!(
        errors[13].message,
        "value 500 does not fit into type uint8."
    );
    assert_eq!(
        errors[14].message,
        "value 300 does not fit into type uint8."
    );
    assert_eq!(errors.len(), 15);
}

#[test]
fn try_catch_solana() {
    let file = r#"
    contract aborting {
    function abort() public returns (int32, bool) {
        revert("bar");
    }
}

contract runner {
    function test() public pure {
        aborting abort = new aborting();

        try abort.abort() returns (int32 a, bool b) {
            // call succeeded; return values are in a and b
        }
        catch Error(string x) {
            if (x == "bar") {
                // "bar" reason code was provided through revert() or require()
            }
        }
        catch (bytes raw) {
            // if no error string could decoding, we end up here with the raw data
        }
    }
}
    "#;

    let mut cache = FileResolver::new();
    cache.set_file_contents("test.sol", file.to_string());

    let ns = parse_and_resolve(OsStr::new("test.ola"), &mut cache);

    assert_eq!(ns.diagnostics.len(), 3);
    assert!(ns.diagnostics.contains_message("found contract 'runner'"));
    assert!(ns.diagnostics.contains_message("found contract 'aborting'"));
    assert!(ns.diagnostics.contains_message("The try-catch statement is not \
     supported on Solana. Please, go to \
     https://solang.readthedocs.io/en/latest/language/statements.html#try-catch-statement for more information"));
}
