use std::ptr::{write_volatile, read_volatile};
use std::{thread, time};

static mut PORTB: u8 = 0x00;
static mut DDRDB: u8 = 0x00;

const CLOCK_PIN: u8 = 5;
const DATA_PIN: u8 = 3;
const TICK: u8 = 500;  // in milliseconds

fn tick() {
    thread::sleep(time::Duration::from_millis(TICK as u64));
}

fn read_bit(byte: u8, position: u8) -> u8 {
    let mask = (1 as u8) << position;
    (byte&mask) >> position
}

fn read_bit_from_register(register: *mut u8, pin_num: u8) -> u8 {
    let state: u8;
    unsafe {
        state = read_volatile(register);
    };
    read_bit(state, pin_num)
}

fn write_bit_to_registry(register: *mut u8, pin_num: u8, value: u8) {
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

fn write_pin(pin_num: u8, value: u8) {
    let portb: *mut u8;
    unsafe { portb = &mut PORTB as *mut u8; };

    write_bit_to_registry(portb, pin_num, value);
}

fn toggle_clock() {
    write_pin(
        CLOCK_PIN,
        match read_pin(CLOCK_PIN) {
            0 => 1,
            1 => 0,
            _ => panic!(),
    });
}


fn read_pin(pin_num: u8) -> u8 {
    let portb: *mut u8;
    unsafe { portb = &mut PORTB as *mut u8; }

    read_bit_from_register(portb, pin_num)
}

fn print_state() {
    let ddrdb: *mut u8;
    unsafe { ddrdb = &mut DDRDB as *mut u8; }

    println!("PORTB: {}    Data Direction Registry (DDRDB): {}",
             (0..8).map(|i| read_pin(i).to_string()).collect::<String>(),
             (0..8).map(|i| read_bit_from_register(ddrdb, i).to_string()).collect::<String>(),
    );
}

fn add_data_to_buffer(byte: u8, buffer: &mut Vec<u8>) {
    for i in (0..8).rev() {
        buffer.push(read_bit(byte, i));
    }
}

fn initialise() {
    println!("BOOT UP... Initial state:");
    println!();
    print_state();
    println!();

    tick();
    tick();
    tick();
    tick();

    let ddrdb: *mut u8;
    unsafe { ddrdb = &mut DDRDB as *mut u8; };
    write_bit_to_registry(ddrdb, DATA_PIN, 1);  // set DATA pin to output
    write_bit_to_registry(ddrdb, CLOCK_PIN, 1);  // set CLOCK pin to output
}

fn send_buffer(buffer: &mut Vec<u8>) {
    let portb: *mut u8;
    unsafe { portb = &mut PORTB as *mut u8; };

    initialise();

    for (i, bit) in buffer.iter().enumerate() {
        if i%8 == 0 {
            println!("acknowledge...");
            toggle_clock();
            tick();
        }

        write_pin(DATA_PIN, *bit as u8);
        toggle_clock();

        print_state();
        tick();
    }
}

fn main() {
    let portb: *mut u8;
    unsafe { portb = &mut PORTB as *mut u8; };
    let mut buffer: Vec<u8> = vec!();

    add_data_to_buffer(21, &mut buffer);
    add_data_to_buffer(37, &mut buffer);
    add_data_to_buffer(14, &mut buffer);
    add_data_to_buffer(88, &mut buffer);
    send_buffer(&mut buffer);
}
