use x86::current::paging::{PDFlags, PDPTFlags};

use crate::{subsystems::SUBSYSTEMS, terminal::Console, assembly_macros, memory::paging::x86::{PDPTTable, PDTable, RootTable}};

pub fn print_diagnostics() {
    SUBSYSTEMS.write().console.as_mut().unwrap().println("Diagnostics result:");
    print_vendor_id();
    dump_page_tables();
}

fn print_vendor_id() {
    SUBSYSTEMS.write().console.as_mut().unwrap().print("- CPU vendor: ");

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

    SUBSYSTEMS.write().console.as_mut().unwrap().println("");
}

fn dump_page_tables() {
    // SUBSYSTEMS.write().console.as_mut().unwrap().print("- Page tables:");
    // let pml4 = SUBSYSTEMS.read().paging.as_ref().unwrap().tables.entries[0];
    // let paging_opt = &SUBSYSTEMS.read().paging;
    // let paging = paging_opt.as_ref().unwrap();
    // let tables = &paging.tables;
    // let mut level = 1;

    // print_indent(&level);
    // SUBSYSTEMS.write().console.as_mut().unwrap().print("PML4: ");
    // SUBSYSTEMS.write().console.as_mut().unwrap().print_hex(&pml4.0);
    // SUBSYSTEMS.write().console.as_mut().unwrap().print(" addr: ");
    // SUBSYSTEMS.write().console.as_mut().unwrap().print_hex(&pml4.address().as_u64());

    // print_indent(&level);
    // SUBSYSTEMS.write().console.as_mut().unwrap().print("PDPTS: ");

    // for page in pdpts.entries.iter() {
    //     if page.is_present() {
    //         let huge = page.flags().contains(PDPTFlags::PS);
    //         level += 1;
    //         print_page(&level, &page.0, &page.address().as_u64(), page.flags().contains(PDPTFlags::PS));

    //         if !huge {
    //             print_indent(&level);
    //             SUBSYSTEMS.write().console.as_mut().unwrap().print("PDS: ");
    //             let pds = unsafe { &*(page.address().as_usize() as *const PDPTTable) }.entries;

    //             for page in pds.iter() {
    //                 if page.is_present() {
    //                     let huge = page.flags().contains(PDFlags::PS);
    //                     level += 1;
    //                     print_page(&level, &page.0, &page.address().as_u64(), huge);

    //                     if !huge {
    //                         print_indent(&level);
    //                         SUBSYSTEMS.write().console.as_mut().unwrap().print("PTS: ");
    //                         let pts = unsafe { &*(page.address().as_usize() as *const PDTable) }.entries;

    //                         for page in pts[0..3].iter() {
    //                             if page.is_present() {
    //                                 level += 1;
    //                                 print_page(&level, &page.0, &page.address().as_u64(), false);
    //                                 level -= 1;
    //                             }
    //                         }
    //                     }
    //                     level -= 1;
    //                 }
    //             }
    //         }
    //         level -= 1;
    //     }
    // }
}

fn print_page(indent: &u8, page: &u64, addr: &u64, huge: bool) {
    print_indent(indent);
    
    SUBSYSTEMS.write().console.as_mut().unwrap().print_hex(page);
    if huge {
        SUBSYSTEMS.write().console.as_mut().unwrap().print(" HUGE ");
    }
    SUBSYSTEMS.write().console.as_mut().unwrap().print(" addr: ");
    SUBSYSTEMS.write().console.as_mut().unwrap().print_hex(addr);
}

fn print_indent(indent: &u8) {
    SUBSYSTEMS.write().console.as_mut().unwrap().println("");
    for _ in 0..indent * 2 {
        SUBSYSTEMS.write().console.as_mut().unwrap().print_char(' ');
    }
    SUBSYSTEMS.write().console.as_mut().unwrap().print("- ");
}