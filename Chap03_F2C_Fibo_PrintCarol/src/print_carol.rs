// use std::io;

// print th lyrics to  "The twelve days of Christmas"
// using the repetition of the song to minimaze the code.
// We go only until the fourth strophe
pub fn run(){
    let days = ["first", "second","third", "fourth"];
    let first_verse_a = "On the";
    let first_verse_b = "day of Christmas,";    
    let last_verse = " patridge in a pear trees.";
    let center_verses = ["","Two turtle doves,", "Three French hens," ]
    let mut prv_str = "";

    for i in 0..4{
        println!("{} {} {}", first_verse_a, days[i],first_verse_b );
        println!("my true love sent to me");
        if i == 0{
            println!("A {}\n", last_verse);
        } else {
            prv_str =  center_verses[i] + "\n" + prv_str;
            println!("{}",prv_str);
            println!("And {}\n", last_verse);
        }

    }


}
/* Next:
*
*/

/* The Music: 

On the first day of Christmas,
my true love sent to me
A partridge in a pear tree.

On the second day of Christmas,
my true love sent to me
Two turtle doves,
And a partridge in a pear tree.

On the third day of Christmas,
my true love sent to me
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fourth day of Christmas,
my true love sent to me
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fifth day of Christmas,
my true love sent to me
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the sixth day of Christmas,
my true love sent to me
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the seventh day of Christmas,
my true love sent to me
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eighth day of Christmas,
my true love sent to me
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the ninth day of Christmas,
my true love sent to me
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the tenth day of Christmas,
my true love sent to me
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eleventh day of Christmas,
my true love sent to me
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the twelfth day of Christmas,
my true love sent to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree!
*/