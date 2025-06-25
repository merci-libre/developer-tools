pub mod chmod_lib {

    fn position_correction(
        inp_vectr: &mut Vec<char>,
        original_vectr: &mut Vec<char>,
        input: char,
        correction: char,
        position: usize,
    ) {
        if input != correction {
            ()
        } else if original_vectr.iter().next().unwrap() != &correction
            && (inp_vectr[position].ne(&'-'))
        {
            inp_vectr[position] = input
        } else {
            inp_vectr[position] = input;
        }
    }
    impl StringFormat for Vec<char> {
        fn format_chmod_value(&mut self) -> Vec<char> {
            let mut fillchars: Vec<char> = vec!['-'; 9];
            let part = Vec::from_iter(self[0..self.len()].iter().cloned());
            let mut truncated: Vec<char> = vec!['0'; 9];

            match self.len() {
                0 => truncated = fillchars.clone(),
                0..4 => {
                    for (_i, j) in self.clone().into_iter().enumerate() {
                        let (correction, position) = match j {
                            'r' => (&j, 0),
                            'w' => (&j, 1),
                            'x' => (&j, 2),
                            _ => todo!(),
                        };
                        position_correction(&mut fillchars, self, j, *correction, position);
                    }
                    return fillchars;
                }
                4..9 => truncated = part,
                _ => truncated.copy_from_slice(&self[0..9]),
            }

            /*to-do:
             * - memorize positions of each inputted character in `tracker`
             *
             * - from the position and character, assume the slots for the inputted char
             *   i.e. `rwx^.xw^rw.` should imply `rwx^-wx^rw-`
             *
             * - if the input is something like `rrr^www^xxx`, format it such that its only `r--^-w-^--x`
             * |-> can be represented as
             *
             * P(r)=i%3->0
             * P(w)=i%3->1
             * P(x)=i%3->2
             *
             * */

            let iter = truncated.into_iter().enumerate();
            for (position, chars) in iter {
                let (correction, j) = match position % 3 {
                    0 => ('r', position),
                    1 => ('w', position),
                    2 => ('x', position),
                    _ => todo!(),
                };
                position_correction(&mut fillchars, self, chars, correction, j);
            }

            return fillchars;
        }

        fn vec_to_string(&mut self) -> String {
            return self.iter().collect::<String>();
        }
    }

    pub trait StringFormat {
        fn format_chmod_value(&mut self) -> Vec<char>;
        fn vec_to_string(&mut self) -> String;
    }
}
