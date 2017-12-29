fn main() {
    let mut same_count = 0;
    let mut tri_c: i64 = 1;
    while same_count < 2 {
        tri_c += 1;
        let tri_num: i64 = tri_c*(tri_c + 1)/2;

        let mut pent_c: i64 = 1;
        let mut pent_num: i64 = pent_c*(3*pent_c - 1)/2;
        while pent_num < tri_num {
            pent_c += 1;
            pent_num = pent_c*(3*pent_c - 1)/2;
        }

        if pent_num == tri_num {
            let mut hex_c: i64 = 1;
            let mut hex_num: i64 = hex_c*(2*hex_c - 1);
            while hex_num < tri_num {
                hex_c += 1;
                hex_num = hex_c*(2*hex_c - 1);
            }

            if hex_num == tri_num {
                println!("T({}) = P({}) = H({}) = {}", tri_c, pent_c, hex_c, tri_num);
                same_count += 1;
            }
        }
    }
}
