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

    let s3 = s2;
    // println!("s2-> {}", s2); // Funktioniert nicht da s2 nicht mehr Owner ist

} // Stack wird freigegeben. Speicher auf den s2 und s3 zeigen wird freigegeben

