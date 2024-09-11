#![feature(box_into_inner)]
#![feature(auto_traits)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#[cfg(not(feature = "std"))]
extern crate alloc;
pub mod base_types;
pub mod class_loader;
pub mod class_runner;
//pub mod virtual_machine;

//use crate::class_loader::class::Class;
#[cfg(test)]
mod tests {
    //use crate::class_loader; //, virtual_machine};
    //use crate::virtual_machine;
    //use class_loader::class::Class;
    use crate::class_loader::Class;
    #[cfg(feature = "log")]
    use log::trace;
    #[cfg(feature = "std")]
    use std::fs;

    #[test]
    fn hello_world() -> Result<(), String> {
        #[cfg(feature = "log")]
        simple_logging::log_to_file("hello_world.log", log::LevelFilter::Trace).unwrap();
        let main_class_path = &"classes/HelloMIDlet/HelloMIDlet.class";
        //#[cfg(feature = "log")]
        //trace!("{:}", main_class_path);
        let main_class_data = std::fs::read(main_class_path).expect("CLASS FILE NOT FOUND");
        //#[cfg(feature = "log")]
        //trace!("{:#X?}", main_class_data);
        let main_class_obj = Class::new(main_class_data);

        println!("{:#?}", main_class_obj);
        //let mut virtual_machine = virtual_machine::VirtualMachine::new(main_class_obj);
        //virtual_machine.run();
        Ok(())
    }
}
