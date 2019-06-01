use std::fs::File;
use std::io::Read;


fn main(){
	let filename = "LeerArchivo.rs";
	//Abrir solo en modo lectura
	match File::open(filename){
		Ok(mut file) =>{
			let mut content = String::new();
			file.read_to_string(&mut content).unwrap();

			println!("{}", content);

		},

		Err(error) => {
			println!("Error opening file{}: {}", filename, error);
		},
	}
}