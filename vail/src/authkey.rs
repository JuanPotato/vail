fn factor(pq: u64) -> (u64, u64) {
    let mut rng = rand::thread_rng();

    let mut q = 0;

    for i in 1..3 {
        let w = (rng.gen_range(0, 127) & 15) + 17;
        let mut x = rng.gen_range(1, 1000000000); // lmao
        let mut y = x;

        let lim = 1 << (i + 18);
        for j in 1..lim {
            let mut a = x;
            let mut b = x;
            let mut c = w;

            while b != 0 {
                if (b & 1) != 0 {
                    c += a;

                    if c >= pq {
                        c -= pq;
                    }
                }

                a += a;

                if a >= pq {
                    a -= pq;
                }

                b >>= 1;
            }

            x = c;
            let z = if x < y { y - x } else { x - y };
            q = gcd(z, pq);

            if q != 1 {
                break;
            }

            if (j & (j - 1)) == 0 {
                y = x;
            }
        }
        
        if q > 1 {
            break;
        }
    }

    let p = pq / q;
    return if p < q { (p, q) } else { (q, p) };
}