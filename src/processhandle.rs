/*
Process handler for libstarlight
*/

use std::{mem, str, thread};
use windows::{
    core::Error,
    Win32::{
        Foundation::*,
        System::{Diagnostics::Debug::*, Memory::*, ProcessStatus::*, Threading::*},
    },
};

pub fn wait_for_process(process_name: &str) -> Result<(u32, HANDLE), Error> {
    loop {
        let mut loaded_processes: Vec<u32> = vec![u32::default(); 1536];
        let mut cb_needed: u32 = 0;
        match {
            unsafe {
                EnumProcesses(
                    loaded_processes.as_mut_ptr(),
                    loaded_processes.len() as u32,
                    &mut cb_needed,
                )
            }
        } {
            Err(err) => return Err(err),
            _ => {}
        }
        let c_processes = cb_needed / mem::size_of::<u32>() as u32;
        for x in 0..c_processes {
            let pid = loaded_processes[x as usize];
            let proc = unsafe {
                if let Ok(res) =
                    OpenProcess(PROCESS_ALL_ACCESS, false, loaded_processes[x as usize])
                {
                    res
                } else {
                    continue;
                }
            };

            let mut proc_name = [0; MAX_PATH as usize];
            match { unsafe { GetProcessImageFileNameA(proc, &mut proc_name as &mut [u8]) } } {
                0 => continue,
                _ => {}
            }
            let proc_name_len = proc_name.iter().position(|&r| r == 0).unwrap();
            if let Ok(proc_name_str) = str::from_utf8(&proc_name[0..proc_name_len]) {
                if proc_name_str.ends_with(process_name) {
                    return Ok((pid, proc));
                }
            } else {
                unsafe { if let Err(_) = CloseHandle(proc) {} };
            }
        }
        thread::yield_now();
    }
}

pub fn wait_for_module(proc: HANDLE, module_name: &str) -> Result<(String, HMODULE), Error> {
    loop {
        let mut h_mods: Vec<HMODULE> = vec![HMODULE::default(); 1536];
        let modlist_size = (h_mods.len() * std::mem::size_of::<HMODULE>()) as u32;
        let mut bytes_needed = 0;

        match unsafe {
            EnumProcessModulesEx(
                proc,
                h_mods.as_mut_ptr(),
                modlist_size,
                &mut bytes_needed,
                LIST_MODULES_ALL,
            )
        } {
            Err(_) => continue,
            Ok(_) => {}
        };
        for mod_addr in &h_mods {
            let mut mod_name = vec![0u16; 1024];
            unsafe {
                match GetModuleFileNameExW(proc, *mod_addr, &mut mod_name) {
                    0 => continue,
                    _ => {}
                }
            }
            let mod_name_len = mod_name.iter().position(|&r| r == 0).unwrap();
            let mod_name_str = String::from_utf16_lossy(&mod_name[0..mod_name_len]);
            if mod_name_str.contains(module_name) {
                return Ok((mod_name_str, *mod_addr));
            }
        }
        thread::yield_now();
    }
}

pub fn get_module_info(
    process_handle: HANDLE,
    module_handle: HMODULE,
) -> Result<MODULEINFO, Error> {
    let mut module_info: MODULEINFO = MODULEINFO::default();
    match {
        unsafe {
            GetModuleInformation(
                process_handle,
                module_handle,
                &mut module_info as *mut _,
                std::mem::size_of::<MODULEINFO>() as u32,
            )
        }
    } {
        Err(err) => Err(err),
        _ => Ok(module_info),
    }
}

pub fn dump_module(
    process_handle: HANDLE,
    module_info: MODULEINFO,
) -> Result<(u32, Vec<u8>), Error> {
    let mut dump: Vec<u8> = vec![0; module_info.SizeOfImage as usize];
    let mut bytes_read: usize = 0;
    match {
        unsafe {
            ReadProcessMemory(
                process_handle,
                module_info.lpBaseOfDll,
                dump.as_mut_ptr() as *mut _,
                module_info.SizeOfImage as usize,
                Some(&mut bytes_read as *mut _),
            )
        }
    } {
        Err(err) => Err(err),
        _ => Ok((bytes_read as u32, dump)),
    }
}

pub fn inject_module(
    process_handle: HANDLE,
    module_info: MODULEINFO,
    data: &mut Vec<u8>,
) -> Result<(), Error> {
    let data_ptr: *mut u8 = data.as_mut_ptr();
    let mut old_security = PAGE_PROTECTION_FLAGS::default();
    unsafe {
        VirtualProtectEx(
            process_handle,
            module_info.lpBaseOfDll,
            module_info.SizeOfImage as usize,
            PAGE_EXECUTE_READWRITE,
            &mut old_security as *mut _,
        )?
    };
    unsafe {
        WriteProcessMemory(
            process_handle,
            module_info.lpBaseOfDll,
            data_ptr as *mut _,
            module_info.SizeOfImage as usize,
            None,
        )?
    }
    Ok(())
}
