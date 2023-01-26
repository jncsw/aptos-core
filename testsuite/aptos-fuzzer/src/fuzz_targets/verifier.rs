
// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::FuzzTargetImpl;

use aptos_proptest_helpers::ValueGenerator;
use move_binary_format::file_format::CompiledModule;
use move_bytecode_verifier::{verify_module};

use move_binary_format::{
    errors::VMError
};


#[derive(Clone, Debug, Default)]
pub struct VerifierFuzzer;
impl FuzzTargetImpl for VerifierFuzzer {
    fn description(&self) -> &'static str {
        "Verify the bytecode verifier of Move module"
    }
    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        println!("Generating corpus for target: VerifierFuzzer.");
        println!("We should use the pattern bytecode as input.");
        // let value = gen.generate(any_with::<CompiledModule>(16));
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
                        panic!("Verification successful");

                    },
                    Err(err) => { // Expected error
                        // println!("Verification failed: {:?}", err);
                        print_error_and_exit(&err);
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
