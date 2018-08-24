/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use rand::Rng;

pub fn factor(pq: u64) -> (u64, u64) {
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

    if p < q {
        (p, q)
    } else {
        (q, p)
    }
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;

    while y != 0 {
        let t = y;

        y = x % y;
        x = t;
    }

    x
}
