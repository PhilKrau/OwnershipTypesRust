fn main(){
    let x = 5; // x = 5
    let y = x; // y = 5
    // Speicher im Stack wird kopiert
    println!("x:{} y:{}", x, y);

    // s1 ist Pointer zu in Heap angelegtem String Object
    let s1 = String::from("String1"); // s1 -> "String1"    
    println!("s1 -> {}", s1);


    let s2 = s1; // s2 -> "String1" s1 ungültig
    // Ownership des Heap Speicherbereichs wurde auf s2 übertragen
    /*
    Vergleich C++:
    string* s1 =  new string("String1");
    string* s2 = s1;
    s1 = nullptr;
     */
    
    let s2_clone = s2.clone(); // Kopie des Speicherbereichs von "Hello World!" wird angelegt
    println!("s2 -> {}, s2_clone -> {}", s2, s2_clone);

    take_ownership(&s2); // Ownership wird an Methode übertragen
    println!("s2-> {}", s2); // Funktioniert nicht da s2 nicht mehr Owner ist
   
    let s3 = take_ownership_and_give_back(s2_clone);
    println!("s3 -> {}", s3);

} // Stack wird freigegeben. Speicher auf den s2 und s3 zeigen wird freigegeben

fn take_ownership(string: String){ // 
    println!("take_ownership string -> {}", string);
} // string Variable out of scope --> Speicher wird freigegeben

fn borrow_value(string: &String){
    println!("borrowed value -> {}", string);
} // Referenz wird übergeben --> Speicher wird nicht freigegeben

fn take_ownership_and_give_back(string: String) -> String {
    println!("take_ownership_and_give_back string -> {}", string);
    string // Return Wert gibt Ownership zurück
}
