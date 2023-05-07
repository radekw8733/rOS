use crate::{assembly_macros, terminal::{CONSOLE, GenericConsole}, println, subsystems::SUBSYSTEMS};

pub fn print_diagnostics() {
    println!("Diagnostics result:");
    print_vendor_id();
    dump_page_tables();
}

fn print_vendor_id() {
    CONSOLE.lock().print("- CPU vendor: ");

    let reg = assembly_macros::cpuid(0);

    for x in 0..4 {
        CONSOLE.lock().print_char((reg.0 >> x * 8) as u8 as char);
    }
    for x in 0..4 {
        CONSOLE.lock().print_char((reg.1 >> x * 8) as u8 as char);
    }
    for x in 0..4 {
        CONSOLE.lock().print_char((reg.2 >> x * 8) as u8 as char);
    }

    println!("");
}

fn dump_page_tables() {
    println!("- Page tables:");
    let kmem = SUBSYSTEMS.read();
    let tables = &kmem.paging.as_ref().unwrap().kmem.as_ref().unwrap();
    for x in 0..8 {
        println!("  {:?}", tables.entries[x]);
    }
}