use crate::{subsystems::SUBSYSTEMS, terminal::Console, assembly_macros};

pub fn print_diagnostics() {
    SUBSYSTEMS.write().console.as_mut().unwrap().println("Diagnostics result:");
    SUBSYSTEMS.write().console.as_mut().unwrap().print("    CPU vendor: ");
    print_vendor_id();
    SUBSYSTEMS.write().console.as_mut().unwrap().println("\n");
}

fn print_vendor_id() {
    let reg = assembly_macros::cpuid(0);

    for x in 0..4 {
        SUBSYSTEMS.write().console.as_mut().unwrap().print_char((reg.0 >> x * 8) as u8 as char);
    }
    for x in 0..4 {
        SUBSYSTEMS.write().console.as_mut().unwrap().print_char((reg.1 >> x * 8) as u8 as char);
    }
    for x in 0..4 {
        SUBSYSTEMS.write().console.as_mut().unwrap().print_char((reg.2 >> x * 8) as u8 as char);
    }
}