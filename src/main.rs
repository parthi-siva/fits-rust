use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    println!("FITS Processor");

    let data_path = Path::new("data/FOCx38i0101t_c0f.fits");
    let mut content = match File::open(data_path) {
        Err(e) => panic!("Couldn't open {} : {}", data_path.display(), e),
        Ok(content) => content,
    };

    let mut fits_data = Vec::new();

    match content.read_to_end (&mut fits_data) {
        Err(e) => panic!("Couldn't open {} : {}", data_path.display(), e),
        Ok(bytes) => println!("Succefully read {} bytes from {}", bytes, data_path.display()) 
    }
    
}
