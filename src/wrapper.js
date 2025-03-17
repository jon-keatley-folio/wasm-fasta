
function register(file_input)
{
  const fin = document.getElementById(file_input);
  fin.addEventListener("change", (event) =>{
    console.log(event);
  });
}