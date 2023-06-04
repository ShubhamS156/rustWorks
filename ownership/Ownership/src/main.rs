/* 
Rules:
Every value in rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope the value is dropped.
*/

fn main() {
    // "s" doesn't exist here
    {
        let s = "hello world"; //s is valid here.
        println!("{}",s);
    }//scope ends
    // "s" goes out of scope here.

    //String datatype is a complex data type , meaning it cant be stored on stack
    // and uses heap.
    {
        let s1 = String::from("hello world");
        let s2 = s1;
        //println!("{s1}"); //this will give error, as the value is moved from s1 to s2.
        //rust does shallow copy or MOVE in line 20.
    }
    
    {
        let s1 = String::from("hello world");
        let s2 = s1.clone();
        println!("{}, {}",s1,s2); 
        // no error here as both s1 and s2 are valid becuase we did a deep copy.
    }

    {
        let s1 = String::from("hello world");
        let s2 = &s1[0..5];
    }

    

}
