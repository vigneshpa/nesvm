use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    generate_opcode_match();
}

fn generate_opcode_match() {
    println!("cargo:rerun-if-changed=codegen/source.json");
    println!("cargo:rerun-if-changed=codegen/cycles.json");

    let mut sink = File::create(get_out_dir("/opcode_match.rs")).unwrap();

    let source = File::open("codegen/source.json").unwrap();
    let cycles = File::open("codegen/cycles.json").unwrap();

    let source = serde_json::from_reader::<_, serde_json::Value>(source).unwrap();
    let source = source.as_object().unwrap();
    let cycles = serde_json::from_reader::<_, serde_json::Value>(cycles).unwrap();
    let cycles = cycles.as_object().unwrap();

    let insmap = source.get("instructions").unwrap().as_object().unwrap();

    writeln!(&mut sink, "match code {{").unwrap();

    for (ins, details) in insmap {
        let modes = details
            .as_object()
            .unwrap()
            .get("opcodes")
            .unwrap()
            .as_object()
            .unwrap();
        for (mode, opcode) in modes {
            let mode = match source.get("modes").unwrap().get(mode) {
                Some(mode) => mode.as_str().unwrap(),
                None => mode,
            };
            let opcode = opcode.as_str().unwrap();
            let cycles = cycles.get(opcode).unwrap().as_i64().unwrap();
            writeln!(
                &mut sink,
                "    0x{} => Opcode {{ instruction: {}, mode: {}, cycles: {} }},",
                opcode, ins, mode, cycles
            )
            .unwrap();
        }
    }

    writeln!(&mut sink, "    _ => panic!(\"Unknown Opcode\")").unwrap();
    writeln!(&mut sink, "}}").unwrap();
}

fn get_out_dir(path: &str) -> String {
    let mut out_dir = env::var("OUT_DIR").unwrap();
    out_dir.push_str(path);
    out_dir
}
