use core::slice::SlicePattern;
use core::str;
use std::io::Write;
use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use noodles::fasta::Reader;
use noodles::fasta::io::Indexer;
use noodles::bgzf::Writer;
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

fn create_fasta_index(fasta:&str) -> Result<String, i32>
{
    let fasta_in:String = fasta.to_owned();
    let fasta_bytes = fasta_in.into_bytes();
    
    let mut indexer = Indexer::new(fasta_bytes.as_slice());
    let mut results:String = String::new();
    let mut error_count = 0;

    while let Ok(rec_result) = indexer.index_record()
    {
        match rec_result 
        {
            Some(rec) =>
            { 
                let name_result = str::from_utf8(rec.name());
                
                if let Ok(name) = name_result
                {
                    results = format!("{} {} {} {} {}\n{}",
                        name,
                        rec.length(),
                        rec.offset(),
                        rec.line_bases(),
                        rec.line_width(),
                        results);
                }
                else
                {
                    error_count += 1;
                }
            }
            None =>
            {
                break
            }
        }

    }
    
    if error_count > 0
    {
        return Err(error_count)
    }
    
    Ok(results)
}

fn create_compressed_fasta(fasta:&str) -> Result<String, String>
{
    let fasta_in:String = fasta.to_owned();
    let fasta_bytes = fasta_in.into_bytes();
    let compressed_fasta_bytes:Vec<u8> = Vec::new();
    
    let mut writer = Writer::new(compressed_fasta_bytes);
    let write_result = writer.write_all(fasta_bytes.as_slice());
    
    if write_result.is_ok()
    {
        //let byte_array = 
        //return Ok(compressed_fasta_bytes.as_slice())
    }
    
    Err("Failed to compress fasta".to_string())
}

#[wasm_bindgen]
pub fn process_file_as_string(f: String)
{
    
    web_sys::console::log_1(&"WASM got this!".into());
    //web_sys::console::log_1(&f.clone().into());
    
    let index_result = create_fasta_index(&f);
    web_sys::console::log_1(&"Indexer complete".into());
    if let Ok(index) = index_result
    {
        web_sys::console::log_1(&"success".into());
        web_sys::console::log_2(&"Index".into(), &index.into());
    }
    else
    {
        web_sys::console::log_1(&"Indexer failed :(".into());
    }
    
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