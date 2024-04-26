fn main(){
    let x = 5; // x = 5
    let y = x; // y = 5
    // Speicher im Stack wird kopiert
    println!("x:{} y:{}", x, y);

    // s1 ist Pointer zu in Heap angelegtem String Object
    let s1 = String::from("String1"); // s1 -> "Hello World"    
    
    let s1_clone = s1.clone(); // Kopie des Speicherbereichs von "Hello World!" wird angelegt
    println!("s1 -> {}, s1_clone -> {}", s1, s1_clone);
    let s2 = s1; // s2 -> "Hello World" s1 ungültig
    // Ownership des Heap Speicherbereichs wurde auf s2 übertragen

    println!("s2 -> {}", s2);
    take_ownership(s2); // Ownership wird an Methode übertragen
    // println!("{}", s2); // Funktioniert nicht da s2 nicht mehr Owner ist
   
    let s3 = take_ownership_and_give_Back(s1_clone);
    println!("s3 -> {}", s3);

} // Stack wird freigegeben. Speicher auf den s2 zeigt wird freigegeben da s2 out of scope

fn take_ownership(string: String){ // 
    println!("take_ownership string -> {}", string);
} // string out of scope --> Speicher wird freigegeben

fn take_ownership_and_give_Back(string: String) -> String {
    println!("take_ownership_and_give_back string -> {}", string);
    string // Return Wert gibt Ownership zurück
}
