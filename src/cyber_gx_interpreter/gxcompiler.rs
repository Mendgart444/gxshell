use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, TargetTriple};
use inkwell::OptimizationLevel;
use std::process::Command;
use std::fs;

pub fn compile(source_code: &str, output_filename: &str) {
    let context = Context::create();
    let module = context.create_module("gx_compiler");
    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let entry = context.append_basic_block(function, "entry");

    builder.position_at_end(entry);
    let return_value = i32_type.const_int(42, false); // Beispiel: Programm gibt 42 zurück
    builder.build_return(Some(&return_value));

    Target::initialize_all(&InitializationConfig::default());
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::None,
            inkwell::targets::RelocMode::Default,
            inkwell::targets::CodeModel::Default,
        )
        .unwrap();

    let file_type = inkwell::targets::FileType::Object;
    let obj_filename = format!("{}.o", output_filename);
    machine.write_to_file(&module, file_type, std::path::Path::new(&obj_filename)).unwrap();

    // Linken mit Clang
    let _ = Command::new("clang")
        .args([&obj_filename, "-o", output_filename])
        .output()
        .expect("Failed to link the object file");

    fs::remove_file(obj_filename).ok(); // Temporäre Datei löschen
}
