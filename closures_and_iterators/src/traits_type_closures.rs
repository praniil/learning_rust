struct Context<T> {
    data: T,
}

impl<T> Context <T> {
    fn execture_once<F>(self, f:F) 
    where F: FnOnce(T), 
    {
        f(self.data);
    }

    fn execute_mut<F>(&mut self, mut f: F) 
    where F: FnMut(&mut T),
    {
        f(&mut self.data);
    }

    fn execute_immut<F>(&self, f: F)
    where F: Fn(&T),
    {
        f(&self.data)
    }
}

pub fn traits_closures() {
    let context = Context{
        data: String::from("Hello, Rusty Gang!"),
    };
    
    //can only be called once because it takes the ownership of the data
    context.execture_once(|data| {
        println!("FnOnce closure: {}", data)
    });

    let mut context_mut = Context{
        data: String::from("hello, Rusty gang from mut context!"),
    };

    context_mut.execute_mut(|vara| {
        vara.push('d');
        println!("FnMut closure: {}", vara);
    });

    let context_imu = Context {
        data: String::from("heyyy"),
    };
    context_imu.execute_immut(|new_var| {
        println!("imu closure: {}", new_var);
    })
}