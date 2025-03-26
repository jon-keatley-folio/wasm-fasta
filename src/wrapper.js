
function register(file_input)
{
  const fin = document.getElementById(file_input);
  fin.addEventListener("change", (event) =>{
    const file = event.target.files[0];
    
    if(!file)
    {
      console.log("No file selected");
      return false;
    }

    if(!file.name.endsWith("fa"))
    {
      console.log("File must be a fasta");
      return false;
    }
    
    const reader = new FileReader();
    reader.onload = () => {
      //console.log(reader.result); 
      //TODO add wasm function here
      window.wasmBindings.process_file_as_string(reader.result);
    };
    
    reader.onerror = () => {
      console.log("Failed to read file! :(");
    }
    
    reader.readAsText(file);
    
  });
}