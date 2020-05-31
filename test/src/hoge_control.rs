#[allow(dead_code)]


pub mod hoge_control {

    use rand::Rng;

    const MAX_CALL_STACK: usize = 10;

    #[derive(PartialEq,Clone,Copy)]
    enum State {
        #[allow(non_camel_case_types)]
        None,
        //    [STATEGO OUTPUT START] indent(8) $/^S_/->#enums$
        //             psggConverterLib.dll converted from psgg-file:hoge_control.psgg

        #[allow(non_camel_case_types)]
            S_0001,
        #[allow(non_camel_case_types)]
            S_0002,
        #[allow(non_camel_case_types)]
            S_0003,
        #[allow(non_camel_case_types)]
            S_0004,
        #[allow(non_camel_case_types)]
            S_0005,
        #[allow(non_camel_case_types)]
            S_0006,
        #[allow(non_camel_case_types)]
            S_0007,
        #[allow(non_camel_case_types)]
            S_END,
        #[allow(non_camel_case_types)]
            S_GSB001,
        #[allow(non_camel_case_types)]
            S_LOP000,
        #[allow(non_camel_case_types)]
            S_LOP000_LoopCheckAndGosub____,
        #[allow(non_camel_case_types)]
            S_LOP000_LoopNext____,
        #[allow(non_camel_case_types)]
            S_RET000,
        #[allow(non_camel_case_types)]
            S_RET001,
        #[allow(non_camel_case_types)]
            S_SBS000,
        #[allow(non_camel_case_types)]
            S_SBS001,
        #[allow(non_camel_case_types)]
            S_START,


        //    [STATEGO OUTPUT END]
    }

    //    [STATEGO OUTPUT START] indent(4) $/^E_/$ 
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

        m_cnt: i32,







        m_loopindex : i32,


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

                m_cnt: 0,







                m_loopindex: 0,


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

        fn start(&mut self) {
            self.m_next = State::S_START;
        }
        fn is_end(&self) -> bool {
            return self.m_cur == State::S_END;
        }
        fn goto(&mut self, s : State) {
            self.m_next = s;
        }
        fn has_next(&self) -> bool {
            self.m_next != State::None
        }
        fn no_wait(&mut self) {
            self.m_nowait = true;
        }

        fn update(&mut self) {
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

                        State::S_0001 => self.S_0001(),
                        State::S_0002 => self.S_0002(),
                        State::S_0003 => self.S_0003(),
                        State::S_0004 => self.S_0004(),
                        State::S_0005 => self.S_0005(),
                        State::S_0006 => self.S_0006(),
                        State::S_0007 => self.S_0007(),
                        State::S_END => self.S_END(),
                        State::S_GSB001 => self.S_GSB001(),
                        State::S_LOP000 => self.S_LOP000(),
                        State::S_LOP000_LoopCheckAndGosub____ => self.S_LOP000_LoopCheckAndGosub____(),
                        State::S_LOP000_LoopNext____          => self.S_LOP000_LoopNext____(),
                        State::S_RET000 => self.S_RET000(),
                        State::S_RET001 => self.S_RET001(),
                        State::S_SBS000 => self.S_SBS000(),
                        State::S_SBS001 => self.S_SBS001(),
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
            S_0001
        */
        #[allow(non_snake_case)]
        fn S_0001(&mut self) {
            if self.m_first {
                println!("StateGo is running");
            }
            if !self.has_next() {
                self.goto(State::S_LOP000);
            }
        }
        /*
            S_0002
            members      : m_cnt: i32,
            members init : m_cnt: 0,
        */
        #[allow(non_snake_case)]
        fn S_0002(&mut self) {
            if self.m_first {
                self.m_cnt = 0;
                println!(">Loop in a state");
            }
            println!("cnt={}", self.m_cnt);
            self.m_cnt += 1;
            if self.m_cnt != 10 {
                return;
            }
            println!(">Loop end");
            if !self.has_next() {
                self.goto(State::S_0003);
            }
        }
        /*
            S_0003
        */
        #[allow(non_snake_case)]
        fn S_0003(&mut self) {
            let num = rand(1000);
            let b = (num % 2) == 0;
            println!("rand num={}", num);
            if b { self.goto( State::S_0004 ); }
            else { self.goto( State::S_0005 ); }
        }
        /*
            S_0004
        */
        #[allow(non_snake_case)]
        fn S_0004(&mut self) {
            if self.m_first {
                println!("---> Even");
            }
            if !self.has_next() {
                self.goto(State::S_END);
            }
        }
        /*
            S_0005
        */
        #[allow(non_snake_case)]
        fn S_0005(&mut self) {
            if self.m_first {
                println!("--->Odd");
            }
            if !self.has_next() {
                self.goto(State::S_END);
            }
        }
        /*
            S_0006
        */
        #[allow(non_snake_case)]
        fn S_0006(&mut self) {
            if self.m_first {
                println!("> You are in the subroutine!");
            }
            if !self.has_next() {
                self.goto(State::S_RET000);
            }
        }
        /*
            S_0007
        */
        #[allow(non_snake_case)]
        fn S_0007(&mut self) {
            if self.m_first {
                println!("> You are in the loop subroutine, cnt = {}", self.m_loopindex);
            }
            if !self.has_next() {
                self.goto(State::S_RET001);
            }
        }
        /*
            S_END
        */
        #[allow(non_snake_case)]
        fn S_END(&mut self) {
        }
        /*
            S_GSB001
        */
        #[allow(non_snake_case)]
        fn S_GSB001(&mut self) {
            self.gosub_state(State::S_SBS000, State::S_0002);
            self.no_wait();
        }
        /*
            S_LOP000
            members      : m_loopindex : i32,
            members init : m_loopindex: 0,
        */
        #[allow(non_snake_case)]
        fn S_LOP000(&mut self) {
            self.m_loopindex = 0;
            self.goto(State::S_LOP000_LoopCheckAndGosub____);
            self.no_wait();
        }
        #[allow(non_snake_case)]
        fn S_LOP000_LoopCheckAndGosub____(&mut self) {
            if self.m_loopindex < 10 { self.gosub_state(State::S_SBS001,State::S_LOP000_LoopNext____); }
            else             { self.goto(State::S_GSB001); }
            self.no_wait();
        }
        #[allow(non_snake_case)]
        fn S_LOP000_LoopNext____(&mut self) {
            self.m_loopindex += 1;
            self.goto(State::S_LOP000_LoopCheckAndGosub____);
            self.no_wait();
        }
        /*
            S_RET000
        */
        #[allow(non_snake_case)]
        fn S_RET000(&mut self) {
            self.return_state();
            self.no_wait();
        }
        /*
            S_RET001
        */
        #[allow(non_snake_case)]
        fn S_RET001(&mut self) {
            self.return_state();
            self.no_wait();
        }
        /*
            S_SBS000
        */
        #[allow(non_snake_case)]
        fn S_SBS000(&mut self) {
            if !self.has_next() {
                self.goto(State::S_0006);
            }
        }
        /*
            S_SBS001
        */
        #[allow(non_snake_case)]
        fn S_SBS001(&mut self) {
            if !self.has_next() {
                self.goto(State::S_0007);
            }
        }
        /*
            S_START
        */
        #[allow(non_snake_case)]
        fn S_START(&mut self) {
            self.goto(State::S_0001);
            self.no_wait();
        }


        //    [STATEGO OUTPUT END]
    }
    fn rand(n: i32) -> i32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0,n+1);
    }

}
