fn main() {
    let n = 4; // Setting n to 4 – think of it like you’ve got 4 pastries to stack before you start eating them one by one.

    // First half of the pattern (increasing pastries)
    for i in 1..=n { // Loop from 1 to n – piling up the pastries nicely, one, then two, then three, then four.
        for _ in 0..i { // This loop helps us place i pastries in each row.
            print!("* "); // One pastry added to the line. Mmm, looking tastier!
        }
        println!(); // Move to the next line – each row deserves its own spotlight, after all.
    }

    // Second half of the pattern (decreasing pastries)
    for i in (1..n).rev() { // Loop from n-1 down to 1, like when everyone starts taking the pastries one by one.
        for _ in 0..i { // For each row, print i pastries – now we’re distributing them.
            print!("* "); // One pastry taken away, fewer pastries left on the plate.
        }
        println!(); // Move to the next line – time to distribute the next round.
    }
}
