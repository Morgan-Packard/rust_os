use x86_64::{structures::idt::{InterruptDescriptorTable, InterruptStackFrame}, 
    instructions::port::Port};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;

use crate::{println, print};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer as usize]
            .set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as usize]
            .set_handler_fn(keyboard_handler);

        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Hit a breakpoint!");
    println!("Stack frame: {:#?}", stack_frame);
}

pub fn load_idt() {
    IDT.load();
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { 
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) 
    });

extern "x86-interrupt" fn timer_handler(stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(
                InterruptIndex::Timer as u8
            );
    }
} 

use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

lazy_static! {
    static ref KEYBOARD: spin::Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
        spin::Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, 
        HandleControl::Ignore)
    );
}

extern "x86-interrupt" fn keyboard_handler(stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let mut scancode: u8 = unsafe {port.read() };

    let key_event = keyboard.add_byte(scancode);
    if key_event.is_ok() {
        let key_event = key_event.unwrap();
        if key_event.is_some() {
            let key_event = key_event.unwrap();
            let key = keyboard.process_keyevent(key_event);
            if key.is_some() {
                let key = key.unwrap();
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),                
                }
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(
                InterruptIndex::Timer as u8
            );
    }
} 