#![no_std]

use gstd::{debug, exec, msg, prelude::*};
use tmg1_io::{Tamagotchi, TmgAction, TmgEvent};

static mut ENTITY: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Error in loading name");
    let birth = exec::block_timestamp();

    unsafe {
        ENTITY = Some(Tamagotchi::new(name, birth));
    }
}

#[no_mangle]
extern "C" fn handle() {
    let input_message: TmgAction = msg::load().expect("Error in loading TmgAction");
    let entity: &Tamagotchi = unsafe { ENTITY.as_ref().expect("The contract is not initialized") };

    match input_message {
        TmgAction::Name => {
            let name = entity.name.clone();
            debug!("Name: {}", &name);
            msg::reply(TmgEvent::Name(name), 0).expect("Error in sending reply");
        }
        TmgAction::Age => {
            let now = exec::block_timestamp();
            let birth = entity.date_of_birth;
            let age = now - birth;
            debug!("{}-{} Age: {}", now, birth, age);
            msg::reply(TmgEvent::Age(age), 0).expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let entity: &Tamagotchi = unsafe { ENTITY.as_ref().expect("The contract is not initialized") };
    msg::reply(entity, 0).expect("Error in sending reply");
}
