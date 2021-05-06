// systemf - c-like syscall macro
// aka an interface of ::Process
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

#[macro_export]
macro_rules! systemf {
    ($arg: expr) => {
        let args: Vec<_> = ($arg).split(' ').collect();
        let command = args.get(0).unwrap();
        let mut __command = std::process::Command::new(command);

        for __arg in args.iter().skip(1) {
           __command.arg(__arg);
        }

        __command.status()
    };
}

fn main() {
    mod tests {
        #[test]
        fn hello_world() {
            systemf!("echo hello, world!");
        }
    }
}