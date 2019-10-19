fn main() {

    let days = [ "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];
    let presents = [ "Twelve Drummers Drumming", "Eleven Pipers Piping", "Ten Lords a Leaping", "Nine Ladies Dancing", "Eigth Maids a Milking", "Seven Swans a Swimming", "Six Geese a Laying", "Five Golden Rings", "Four Calling Birds", "Three French Hens", "Two Turtle Doves", "a Patridge in a Pear Tree"];

    for pos in 0..12{
        println!("\nOn the {} day of Christmas my true love sent to me: ", days[pos]);
        for present in 11-pos..12 {
            if (present == 11) && (pos != 0){
                print!("And ");
            }
            println!("{}", presents[present]);
        }
    }
}
