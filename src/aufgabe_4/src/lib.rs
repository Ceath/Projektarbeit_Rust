// Die einzelnen Module müssen hier als pub definiert werden damit sie von einer anderen crate(hier main.rs) verwendet werden können
// In Rust stellt jede Datei in /src (außer lib.rs und main.rs) ein eigenes Module dar
pub mod ast;
pub mod parser;
pub mod tokenizer;
pub mod vm;
pub mod vm_parser;