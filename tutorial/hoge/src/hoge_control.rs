#[allow(dead_code)]


pub mod hoge_control {

    use rand::Rng;

    const MAX_CALL_STACK: usize = 10;

    #[derive(PartialEq,Clone,Copy)]
    pub enum State {
        #[allow(non_camel_case_types)]
        None,
        //    [STATEGO OUTPUT START] indent(8) $/^S_/->#enums$
        //             psggConverterLib.dll converted from psgg-file:hoge_control.psgg

        #[allow(non_camel_case_types)]
            S_END,
        #[allow(non_camel_case_types)]
            S_GET_RAND,
        #[allow(non_camel_case_types)]
            S_PRINT_1,
        #[allow(non_camel_case_types)]
            S_PRINT_2,
        #[allow(non_camel_case_types)]
            S_PRINT_3,
        #[allow(non_camel_case_types)]
            S_PRINT_4,
        #[allow(non_camel_case_types)]
            S_PRINT_HELLO,
        #[allow(non_camel_case_types)]
            S_START,


        //    [STATEGO OUTPUT END]
    }

    //    [STATEGO OUTPUT START] indent(4) $/^E_/->#ins_embed$
    //             psggConverterLib.dll converted from psgg-file:hoge_control.psgg



    //    [STATEGO OUTPUT END] 

    pub struct HogeControl {
        m_first: bool,
        m_nowait: bool,
        m_cur:  State,
        m_next: State,
        m_callstack: [State; MAX_CALL_STACK],
        m_callstack_level: usize,

        //    [STATEGO OUTPUT START] indent(8) $/^S_/->#def_members$
        //             psggConverterLib.dll converted from psgg-file:hoge_control.psgg

        m_val : i32,

        //    [STATEGO OUTPUT END] 
    }

    impl HogeControl {
        pub fn new() -> HogeControl {
            HogeControl {
                m_first: false,
                m_nowait: false,
                m_cur: State::None,
                m_next: State::None,
                m_callstack: [State::None; MAX_CALL_STACK],
                m_callstack_level: 0,
                //    [STATEGO OUTPUT START] indent(16) $/^S_/->#ini_members$
                //             psggConverterLib.dll converted from psgg-file:hoge_control.psgg

                m_val : 0,

                //    [STATEGO OUTPUT END] 
            }
        }
        pub fn run(&mut self) {
            println!("@run");
            self.start();
            loop {
                self.update();
                if self.is_end() {
                     break;    
                }
            }
            println!("@exit");
        }

        pub fn start(&mut self) {
            self.m_next = State::S_START;
        }
        pub fn is_end(&self) -> bool {
            return self.m_cur == State::S_END;
        }
        pub fn goto(&mut self, s : State) {
            self.m_next = s;
        }
        pub fn has_next(&self) -> bool {
            self.m_next != State::None
        }
        pub fn no_wait(&mut self) {
            self.m_nowait = true;
        }

        pub fn update(&mut self) {
            loop {
                self.m_nowait = false;
                self.m_first = false;
                if self.m_next != State::None {
                    self.m_cur = self.m_next;
                    self.m_next = State::None;
                    self.m_first = true;            
                }
                if self.m_cur != State::None {
                    match self.m_cur {
                        //    [STATEGO OUTPUT START] indent(24) $/^S_/->#matches$
                        //             psggConverterLib.dll converted from psgg-file:hoge_control.psgg

                        State::S_END => self.S_END(),
                        State::S_GET_RAND => self.S_GET_RAND(),
                        State::S_PRINT_1 => self.S_PRINT_1(),
                        State::S_PRINT_2 => self.S_PRINT_2(),
                        State::S_PRINT_3 => self.S_PRINT_3(),
                        State::S_PRINT_4 => self.S_PRINT_4(),
                        State::S_PRINT_HELLO => self.S_PRINT_HELLO(),
                        State::S_START => self.S_START(),


                        //    [STATEGO OUTPUT END]
                        _ => {},
                    }
                }
                if self.m_nowait == false {
                    break;
                }
            }
        }
        fn gosub_state(&mut self, sub: State, next: State) {
            if self.m_callstack_level >= (MAX_CALL_STACK - 1) {
                panic!("CALL STACK OVER FLOW");
            }
            self.m_callstack[self.m_callstack_level] = next;
            self.m_callstack_level += 1;
            self.goto(sub);
        }
        fn return_state(&mut self) {
            if self.m_callstack_level <= 0 {
                panic!("CALL STACK UNDER FLOW");
            }
            self.m_callstack_level -= 1;
            let next = self.m_callstack[self.m_callstack_level];
            self.goto(next);
        }

        //    [STATEGO OUTPUT START] indent(8) $/^S_/$
        //             psggConverterLib.dll converted from psgg-file:hoge_control.psgg

        /*
            S_END
        */
        #[allow(non_snake_case)]
        fn S_END(&mut self) {
        }
        /*
            S_GET_RAND
            members      : m_val : i32,
            members init : m_val : 0,
        */
        #[allow(non_snake_case)]
        fn S_GET_RAND(&mut self) {
            self.m_val = rand(1,10);
            if self.m_val == 1 { self.goto( State::S_PRINT_1 ); }
            else if self.m_val == 2 { self.goto( State::S_PRINT_2 ); }
            else if self.m_val == 3 { self.goto( State::S_PRINT_3 ); }
            else { self.goto( State::S_PRINT_4 ); }
        }
        /*
            S_PRINT_1
        */
        #[allow(non_snake_case)]
        fn S_PRINT_1(&mut self) {
            if self.m_first {
                println!("You are {}st", self.m_val);
            }
            if !self.has_next() {
                self.goto(State::S_END);
            }
        }
        /*
            S_PRINT_2
        */
        #[allow(non_snake_case)]
        fn S_PRINT_2(&mut self) {
            if self.m_first {
                println!("You are {}nd", self.m_val);
            }
            if !self.has_next() {
                self.goto(State::S_END);
            }
        }
        /*
            S_PRINT_3
        */
        #[allow(non_snake_case)]
        fn S_PRINT_3(&mut self) {
            if self.m_first {
                println!("You are {}rd", self.m_val);
            }
            if !self.has_next() {
                self.goto(State::S_END);
            }
        }
        /*
            S_PRINT_4
        */
        #[allow(non_snake_case)]
        fn S_PRINT_4(&mut self) {
            if self.m_first {
                println!("You are {}th", self.m_val);
            }
            if !self.has_next() {
                self.goto(State::S_END);
            }
        }
        /*
            S_PRINT_HELLO
            定番のHello Worldを表示
        */
        #[allow(non_snake_case)]
        fn S_PRINT_HELLO(&mut self) {
            println!("Hello, World!");
            if !self.has_next() {
                self.goto(State::S_GET_RAND);
            }
        }
        /*
            S_START
        */
        #[allow(non_snake_case)]
        fn S_START(&mut self) {
            self.goto(State::S_PRINT_HELLO);
            self.no_wait();
        }


        //    [STATEGO OUTPUT END]
        
    }
    fn rand(a: i32, b: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let n = b - a;
        return a + rng.gen_range(0,n+1);
    }
}
