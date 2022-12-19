use std::io::Write;

fn main() {
    let lagers = "The lagers produced by Nigerian Brewery Limited are: 33 Export, Desperados, Goldberg, Gulder, Heineken, and Star.\n";
    let stouts = "The Stouts produced by Nigerian Brewery Limited are: Legend, Turbo King, and Williams.\n";
    let non_alcoholics = "The Non-Alcoholics produced by Nigerian Brewery Limited are: Maltina, Amstel Malta, Malta Gold, and Fayrouz.\n";

    let mut file = std::fs::File::create("Nigerian_Brewery.txt").expect("create function failed");
    file.write_all("This is a file which shows the portfolio of Nigerian Brewery Limited.\n".as_bytes()).expect("Failed to write");
    file.write_all(lagers.as_bytes()).expect("Failed to write");
    file.write_all(stouts.as_bytes()).expect("Failed to write");
    
    file.write_all(non_alcoholics.as_bytes()).expect("Failed to write");

    println!("Data has been written into a file")
}