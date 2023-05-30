fn main(){
    let mut row = 0;
    
    while row < 5{
        let mut col = 0;
        while col < row + 1{
            print!("*");
            col += 1;
        }
        println!("");
        row += 1;
    }
}
