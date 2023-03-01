
// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::FuzzTargetImpl;

use aptos_proptest_helpers::ValueGenerator;
use aptos_types::vm_status::VMStatus;
use move_binary_format::file_format::CompiledModule;
use proptest::prelude::*;
use move_bytecode_verifier::{verify_module};

use move_binary_format::{
    errors::VMError
};

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct OverflowFuzzer;
impl FuzzTargetImpl for OverflowFuzzer {
    fn description(&self) -> &'static str {
        "Overflow fuzzer for Aptos VM"
    }
    // fn generate(&self, _idx: usize, gen: &mut ValueGenerator) -> Option<Vec<u8>> {
    //     println!("Generating corpus for target: Overflowfuzzer");
    //     let value = gen.generate(any_with::<CompiledModule>(16));
    //     let mut out = vec![];
    //     value
    //         .serialize(&mut out)
    //         .expect("serialization should work");
    //     Some(out)
    // }

    fn generate(&self, _idx: usize, gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        println!("Generating corpus for target: Overflowfuzzer");

        let mut errMap: HashMap<String, u32> = HashMap::new();

        let mut i = 0;
        while i < 1000{
            let value = gen.generate(any_with::<CompiledModule>(16));
            let res = verify_module(&value);
            match res {
                Ok(_) => {
                    // println!("Verification successful");
                    println!("{:?}", res);
                    *errMap.entry("successTotal".to_string()).or_insert(0) += 1;
                },
                Err(err) => {
                    // println!("Verification failed: {:?}", err);
                    *errMap.entry("failedTotal".to_string()).or_insert(0) += 1;
                    let vmerr: VMStatus = err.into_vm_status();
                    let errcode = format!("{:#?}", vmerr.status_code());
                    *errMap.entry(errcode.to_string()).or_insert(0) += 1;
                }
            }
            i += 1;
        }
        for (key, value) in &errMap {
            println!("{}: {}", key, value);
        }
        errMap.clear();

        // let mut out = vec![];
        // value
        //     .serialize(&mut out)
        //     .expect("serialization should work");
        // Some(out)
        None
    }


    fn fuzz(&self, data: &[u8]) {

        // println!("Fuzzing target: Overflowfuzzer: {:?} bytes", data);
        let module = CompiledModule::deserialize(data);
        match module {
            Ok(module) => {
                // println!("Deserialization successful");
                let veri_res = verify_module(&module); //.unwrap_or_else(|err| print_error_and_exit(&err));
                match veri_res {
                    Ok(_) => { // Could be vulnerable
                        println!("Verification successful");
                        println!("Fuzzing target: Overflowfuzzer: {:?} bytes", data);
                        // panic!("Verification successful");

                    },
                    Err(err) => { // Expected error
                        println!("Verification failed: {:?}", err);
                        // print_error_and_exit(&err);
                    }
                }
            },
            Err(_err) => {
                // println!("Deserialization failed: {:?}", err);
            }
        }

    }

}

fn print_error_and_exit(verification_error: &VMError) -> ! {
    println!("Verification failed:");
    println!("{:?}", verification_error);
    std::process::exit(1);
}
