use std::ptr::{write_volatile, read_volatile};
use std::{thread, time};

static mut PORTB: u8 = 0xFF;
static mut DDRDB: u8 = 0x00;

const CLOCK_PIN: u8 = 5;
const DATA_PIN: u8 = 8;
const TICK: u8 = 50;  // in milliseconds

fn tick() {
    thread::sleep(time::Duration::from_millis(TICK as u64));
}

fn read_bit(register: *mut u8, pin_num: u8) -> u8 {
    let mask = (1 as u8) << pin_num;
    let state: u8;
    unsafe {
        state = read_volatile(register);
    };
    (state&mask) >> pin_num
}

fn write_bit(register: *mut u8, pin_num: u8, value: u8) {
    let mask = (1 as u8) << pin_num;
    unsafe {
        let state = read_volatile(register);
        write_volatile(
            register,
            match value {
                0 => state&(!mask),
                _ => state|mask,
            })
    }
}

fn read_pin(pin_num: u8) -> u8 {
    let portb: *mut u8;
    unsafe { portb = &mut PORTB as *mut u8; }

    read_bit(portb, pin_num)
}

fn print_state() {
    let ddrdb: *mut u8;
    unsafe { ddrdb = &mut DDRDB as *mut u8; }

    println!("PORTB: {}    Data Direction Registry (DDRDB): {}",
             (0..8).map(|i| read_pin(i).to_string()).collect::<String>(),
             (0..8).map(|i| read_bit(ddrdb, i).to_string()).collect::<String>(),
    );
}

fn main() {
    let portb: *mut u8;
    unsafe { portb = &mut PORTB as *mut u8; }


    print_state();
    for i in 0..8 {
        write_bit(portb, i, 0);
        print_state();
    }
}
