fn main() {
    // 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).

    let mut combinations = 0;

    for ones in 0..201 {
        println!("{:?}: {}", ones, combinations);
        for twos in 0..101 {
            for fives in 0..41 {
                for tens in 0..21 {
                    for twenties in 0..11 {
                        for fifties in 0..5 {
                            for hundreds in 0..3 {
                                for twohundreds in 0..2 {
                                    if ones + 2*twos + 5*fives + 10*tens + 20*twenties + 50*fifties + 100*hundreds + 200*twohundreds == 200 {
                                        combinations += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", combinations);
}
