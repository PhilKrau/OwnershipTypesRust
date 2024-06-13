fn main(){
    let mut s = String::from("String 1");

    let r1 = &mut s;
    change(r1);
    printout(r1);
    
    if false{
        let r2 = &mut s;
        change(r2);
        printout(r2);
    }


    printout(r1); // doesn't work because of second mutable borrow
}

fn change(string: &mut String){
    string.push_str(", added something to String");
}

fn printout(string: &String){
    println!("string -> {}", string);
}