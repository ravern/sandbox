struct Vm {
    env: Env,
    fibers: Vec<Fiber>,
}

impl Vm {
    fn new(env: Env) -> Vm {
        Vm {
            env,
            fibers: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.fibers.push(Fiber::new());
    }
}

struct Env {
    value: usize,
}

impl Env {
    fn new(value: usize) -> Env {
        Env { value }
    }
}

struct Fiber;

impl Fiber {
    fn new() -> Fiber {
        Fiber
    }

    fn run(&mut self, env: &mut Env) {}
}

pub fn run() {
    let env = Env::new(1);

    let mut vm = Vm::new(env);
    vm.run();
}
