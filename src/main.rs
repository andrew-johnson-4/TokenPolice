use std::io::BufRead;

struct Transistor {
   state: (char,char),
   transition: (char,char),
}

//closed circuit travelling through binary choices 1,5,2:3,4,6
const DEFAULT_MODEL: [Transistor; 6] = [
   //from ring 1, to ring 5
   Transistor {
      state: ('o','y'),
      transition: ('G','P'),
   },

   //from ring 2, to ring 1
   Transistor {
      state: ('b','r'),
      transition: ('o','y'),
   },

   //from ring 3, to ring 4
   Transistor {
      state: ('p','g'),
      transition: ('W','B'),
   },
 
   //from ring 4, to ring 6
   Transistor {
      state: ('B','W'),
      transition: (':','='),
   },

   //from ring 5, to ring 2
   Transistor {
      state: ('P','G'),
      transition: ('r','b'),
   },

   //from ring 6, to ring 3
   Transistor {
      state: ('=',':'),
      transition: ('g','p'),
   },
];

fn main() {
   //initial state is 1:o,3:p
   let mut state = ('o','p');

   let mut buffer = String::with_capacity(2);
   let mut stdin = std::io::stdin().lock();

   //choices are quaternary
   loop {
      println!("current state: {}, {}", state.0, state.1);

      let mut left = DEFAULT_MODEL[0].transition;
      for tr in DEFAULT_MODEL {
         if tr.state.0==state.0 || tr.state.1==state.0 {
            left = tr.transition;
         }
      }
      let mut right = DEFAULT_MODEL[2].transition;
      for tr in DEFAULT_MODEL {
         if tr.state.0==state.1 || tr.state.1==state.1 {
            right = tr.transition;
         }
      }
      println!("Choose a transition: {}{}{}{}", left.0, left.1, right.0, right.1);

      buffer.clear();
      stdin.read_line(&mut buffer).expect("Failed to read line from standard input");
      let mut cs = buffer.chars();
      if let (Some(c1),Some(c2)) = (cs.next(), cs.next()) {
         state = (c1, c2);
      } else {
         println!("Invalid Response");
      }
   }
}
