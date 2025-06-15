pub struct FunctionInjector {
    builtin_functions_code: String,
}

impl FunctionInjector {
    pub fn new() -> Self {
        Self {
            builtin_functions_code: String::from(
                "
                function abs(x:Number):Number{
                    if (x < 0){
                        -x;
                    }
                    else{
                        x;
                    }
                };

                function sqrt(x:Number):Number {
                    if (x < 0){
                        -1;
                    }
                    elif (x == 0){
                        0;
                    }
                    else{
                        let epsilon = 0.0000000001 in
                        let prev = x, guess = x / 2.0, first = true in
                        while (first | (abs(guess - prev) > epsilon)) {
                            first := false;
                            prev := guess;
                            guess := (guess + x / guess) / 2.0;
                        }
                    }
                };

                function exp(x: Number): Number {
                    if (x == 0) {
                        1;
                    } else {
                        let abs_x = abs(x) in
                        let result = 0, term = 1, n = 1, iter = 0, epsilon = 0.0000000001, max_iter = 1000 in {
                            while ((term >= epsilon) & (iter < max_iter)) {
                                result := result + term;
                                term := term * abs_x / n;
                                n := n + 1;
                                iter := iter + 1;
                            };
                            if (x < 0) {
                                1 / result;
                            } else {
                                result;
                            }
                        }
                    }
                };

                function sin(x: Number): Number {
                    let two_pi = 2 * PI in
                    let reduced_x = x in {
                        while (reduced_x > two_pi) {
                            reduced_x := reduced_x - two_pi;
                        };
                        while (reduced_x < -two_pi) {
                            reduced_x := reduced_x + two_pi;
                        };
                        let result = 0.0, term = reduced_x, n = 0, epsilon = 0.0000000001, max_iter = 1000 in {
                            while ((abs(term) >= epsilon) & (n < max_iter)) {
                                result := result + term;
                                n := n + 1;
                                term := term * (-1) * reduced_x * reduced_x / ((2 * n) * (2 * n + 1));
                            };
                            result;
                        }
                    }
                };

                function cos(x: Number): Number {
                    let two_pi = 2 * PI in
                    let reduced_x = x in {
                        while (reduced_x > two_pi) {
                            reduced_x := reduced_x - two_pi;
                        };
                        while (reduced_x < -two_pi) {
                            reduced_x := reduced_x + two_pi;
                        };
                        let result = 0.0, term = 1, n = 0, epsilon = 0.0000000001, max_iter = 1000 in {
                            while ((abs(term) >= epsilon) & (n < max_iter)) {
                                result := result + term;
                                n := n + 1;
                                term := term * (-1) * reduced_x * reduced_x / ((2 * n - 1) * (2 * n));
                            };
                            result;
                        }
                    }
                };

                function log10(x: Number): Number {
                    if (x <= 0) {
                        0;
                    } else {
                        let int_part = 0 in
                        let temp = x in {
                            while (temp >= 10) {
                                temp := temp / 10;
                                int_part := int_part + 1;
                            };
                            
                            while (temp < 1) {
                                temp := temp * 10;
                                int_part := int_part - 1;
                            };
                            
                            let y = (temp - 1) / (temp + 1) in
                            let y2 = y * y in
                            let frac = 0.0, term = y, n = 0, epsilon = 0.0000000001, max_iter = 1000 in {
                                while ((abs(term) >= epsilon) & (n < max_iter)) {
                                    frac := frac + term;
                                    n := n + 1;
                                    term := term * y2 * (2 * n - 1) / (2 * n + 1);
                                };
                                let fractional = 0.8685889638065035 * 2 * frac in
                                    (int_part + fractional);
                            }
                        }
                    }
                };

                function log(base: Number, value: Number): Number {
                    if (base <= 0 | value <= 0) {
                        0;
                    } else {
                        log10(value) / log10(base);
                    }
                };

                function rand(): Number {
                    let seed = (PI * 1000000000) - 3141592653 in
                    let a = 1664525 in
                    let c = 1013904223 in
                    let m = 4294967296 in 
                    let new_seed = ((a * seed + c) % m) in
                    (new_seed / m);
                };
                ",
            ),
        }
    }

    pub fn get_builtin_functions_code(&self) -> &str {
        &self.builtin_functions_code
    }

    pub fn get_builtin_functions_code_lines(&self) -> usize {
        self.builtin_functions_code.lines().count()
    }

    pub fn inject_code(&self, input: &str) -> String {
        format!("{}\n{}", self.builtin_functions_code, input)
    }
}
