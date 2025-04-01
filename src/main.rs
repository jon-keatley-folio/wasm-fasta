
use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use noodles::fasta::Reader; 
//use web_sys::window;

fn start_app() {
    //let document = window()
    //    .and_then(|win| win.document())
    //    .expect("Could not access document");
    //let body = document.body().expect("Could not access document.body");
    //let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    //body.append_child(text_node.as_ref())
    //    .expect("Failed to append text");
    
    //build download buttons
    
    //on clicks need to pull the desired values
}

#[wasm_bindgen]
pub fn wasm_hello() {
    web_sys::console::log_1(&"Hello from WASM!".into());
}

fn create_compressed_fasta(fasta:&str) -> Result<String, String>
{
    let fasta_in:String = fasta.to_owned();
    let fasta_bytes = fasta_in.into_bytes();
    let mut reader = Reader::new(fasta_bytes.as_slice());
    
    for rec_result in reader.records()
    {
        
    }
    
    Err("WIP".to_string())
}

fn create_fasta_index(fasta:&str) -> Result<String, String>
{
    let fasta_in:String = fasta.to_owned();
    let fasta_bytes = fasta_in.into_bytes();
    let mut reader = Reader::new(fasta_bytes.as_slice());
    
    for rec_result in reader.records()
    {
        
    }

    
    Err("WIP".to_string())
}

#[wasm_bindgen]
pub fn process_file_as_string(f: String)
{
    
    web_sys::console::log_1(&"WASM got this!".into());
    //web_sys::console::log_1(&f.into());
    
    //let fasta_bytes = f.into_bytes().clone();
    //let repo_results =  get_repo(fasta_bytes.as_slice());
    
    //process input into Repository
    // enable download buttons
    // store fasta somewhere
    // onclick of buttons download index or bz zipped fasta
}

fn main() {
    set_panic_hook();
    start_app();
}