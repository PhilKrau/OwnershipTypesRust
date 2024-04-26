fn main(){
    let x = 5; // x = 5
    let y = x; // y = 5
    // Speicher im Stack wird kopiert

    // s1 ist Pointer zu in Heap angelegtem String Object
    let s1 = String::from("Hello World!"); // s1 -> "Hello World"

    let s2 = s1; // s2 -> "Hello World" s1 ungültig
    let s2_clone = s1.clone(); // Kopie des Speicherbereichs von "Hello World!" wird angelegt
    // Ownership des Heap Speicherbereichs wurde auf s2 übertragen
    
    take_ownership(s2); // Ownership wird an Methode übertragen
    // println!("{}", s2); // Funktioniert nicht da s2 nicht mehr Owner ist
   
    let s3 = take_ownership_and_give_Back(s2_clone);
    println!("{}", s3);

} // Stack wird freigegeben. Speicher auf den s2 zeigt wird freigegeben da s2 out of scope

fn take_ownership(string: String){ // 
    println!("{}", string);
} // string out of scope --> Speicher wird freigegeben

fn take_ownership_and_give_Back(string: String) -> String {
    println!("{}", string);
    string // Return Wert gibt Ownership zurück
}
