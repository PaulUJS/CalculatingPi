fn main() {
    let mut x = 2.0;
    let mut y = 3.0;
    let mut z = 4.0;

    let mut pi = 3.0 + 4.0 / (x*y*z);
    
    let mut count = 0;
    let mut flag = 0;

    while count < 1000 {
        println!("{pi}");
        if flag == 0 {
            x += 2.0;
            y += 2.0;
            z += 2.0;
            pi = pi - 4.0 / (x*y*z);
            count +=1;
            flag += 1;
        } else {
            x += 2.0;
            y += 2.0;
            z += 2.0;
            pi = pi + 4.0 / (x*y*z);
            count +=1;
            flag -= 1;
        }
    }
}

