use miden::{Program, ProgramInputs, ProofOptions, StarkProof};
use std::io::Write;
use std::time::Instant;
use structopt::StructOpt;

pub mod fibonacci;

// EXAMPLE
// ================================================================================================

pub struct Example {
    pub program: Program,
    pub inputs: ProgramInputs,
    pub pub_inputs: Vec<u64>,
    pub num_outputs: usize,
    pub expected_result: Vec<u64>,
}

// EXAMPLE OPTIONS
// ================================================================================================

#[derive(StructOpt, Debug)]
#[structopt(name = "Examples", about = "Run an example miden program")]
pub struct ExampleOptions {
    #[structopt(subcommand)]
    pub example: ExampleType,

    /// Security level for execution proofs generated by the VM
    #[structopt(short = "s", long = "security", default_value = "96bits")]
    security: String,
}

#[derive(StructOpt, Debug)]
//#[structopt(about = "available examples")]
pub enum ExampleType {
    /// Compute a Fibonacci sequence of the specified length
    Fib {
        /// Length of Fibonacci sequence
        #[structopt(short = "n", default_value = "1024")]
        sequence_length: usize,
    },
}

impl ExampleOptions {
    pub fn get_proof_options(&self) -> ProofOptions {
        match self.security.as_str() {
            "96bits" => ProofOptions::with_96_bit_security(),
            "128bits" => ProofOptions::with_128_bit_security(),
            other => panic!("{} is not a valid security level", other),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        println!("============================================================");

        // configure logging
        env_logger::Builder::new()
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .filter_level(log::LevelFilter::Debug)
            .init();

        let proof_options = self.get_proof_options();

        // instantiate and prepare the example
        let example = match self.example {
            ExampleType::Fib { sequence_length } => fibonacci::get_example(sequence_length),
        };

        let Example {
            program,
            inputs,
            num_outputs,
            pub_inputs,
            expected_result,
        } = example;
        println!("--------------------------------");

        // execute the program and generate the proof of execution
        let now = Instant::now();
        let (outputs, proof) = miden::prove(&program, &inputs, &proof_options).unwrap();
        println!("--------------------------------");

        println!(
            "Executed program in {} ms",
            //hex::encode(program.hash()), // TODO: include into message
            now.elapsed().as_millis()
        );
        println!("Program output: {:?}", outputs.stack_outputs(num_outputs));
        assert_eq!(
            expected_result,
            outputs.stack_outputs(num_outputs),
            "Program result was computed incorrectly"
        );

        // serialize the proof to see how big it is
        let proof_bytes = proof.to_bytes();
        println!("Execution proof size: {} KB", proof_bytes.len() / 1024);
        println!("Execution proof security: {} bits", proof.security_level(true));
        println!("--------------------------------");

        // verify that executing a program with a given hash and given inputs
        // results in the expected output
        let proof = StarkProof::from_bytes(&proof_bytes).unwrap();
        let now = Instant::now();
        match miden::verify(program.hash(), &pub_inputs, &outputs, proof) {
            Ok(_) => println!("Execution verified in {} ms", now.elapsed().as_millis()),
            Err(err) => println!("Failed to verify execution: {}", err),
        }

        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
pub fn test_example(example: Example, fail: bool) {
    let Example {
        program,
        inputs,
        pub_inputs,
        num_outputs,
        expected_result,
    } = example;

    let (mut outputs, proof) = miden::prove(&program, &inputs, &ProofOptions::default()).unwrap();

    assert_eq!(
        expected_result,
        outputs.stack_outputs(num_outputs),
        "Program result was computed incorrectly"
    );

    if fail {
        outputs.stack_mut()[0] += 1;
        assert!(miden::verify(program.hash(), &pub_inputs, &outputs, proof).is_err())
    } else {
        assert!(miden::verify(program.hash(), &pub_inputs, &outputs, proof).is_ok());
    }
}
