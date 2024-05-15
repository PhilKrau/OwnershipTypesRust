fn main(){
    // s1 ist Pointer zu in Heap angelegtem String Object
    let s1 = String::from("String1"); // s1 -> "String1"    
    println!("s1 -> {}", s1);

    borrow_value(&s1);

    println!("s1 after method call -> {}", s1);

    let s2 = &s1; // s2 ist NICHT Besitzer des Objekts, verweist aber darauf
    println!("s1 -> {}, s2 -> {}", s1, s2);
} // Stack wird freigegeben. Speicher auf den s2 und s3 zeigen wird freigegeben

fn borrow_value(string: &String){
    println!("borrowed value -> {}", string);
} // Referenz wird übergeben --> Speicher wird nicht freigegeben

