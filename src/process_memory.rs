use std::{
	ffi::c_void,
	process::exit,
};

use windows::{
	Win32::{
		Foundation::{CloseHandle, HANDLE},
		System::{
			Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
			Threading::{OpenProcess, PROCESS_ALL_ACCESS},
		},
	},
};

pub(crate) struct ProcessMemory 
{
	handle: HANDLE,
}

impl ProcessMemory
{
	unsafe fn new(pid: u32) -> Self 
	{
		let handle: HANDLE = match OpenProcess(PROCESS_ALL_ACCESS, false, pid) 
		{
			Ok(handle) => handle,
			Err(e) => {
				eprintln!("Could not open process: {:?}", e);
				exit(1);
			}
		};

		ProcessMemory { handle }
	}
	
	pub fn safe_new(pid: u32) -> Self
	{
		unsafe { Self::new(pid) }
	}

	 unsafe fn read_memory
	(
		&self,
		base_addr: *const c_void,
		buffer: *mut c_void,
		size: usize
	) -> Result<usize, String> 
	{
		let mut bytes_read: usize = 0;

		match ReadProcessMemory(self.handle, base_addr, buffer, size, Some(&mut bytes_read)) 
		{
			Ok(_) => Ok(bytes_read),
			Err(e) => Err(format!("Could not read memory: {e}").into()),
		}
	}
	
	pub fn safe_read(
		&self,
		base_addr: *const c_void,
		buffer: *mut c_void,
		size: usize
	) -> Result<usize, String>
	{
		unsafe { self.read_memory(base_addr, buffer, size) }
	}

	unsafe fn write_memory
	(
		&self,
		base_addr: *mut c_void,
		buffer: *const c_void,
		size: usize,
	) -> Result<usize, String> 
	{
		let mut bytes_written: usize = 0;

		match WriteProcessMemory(self.handle, base_addr, buffer, size, Some(&mut bytes_written)) 
		{
			Ok(_) => Ok(bytes_written),
			Err(e) => Err(format!("Could not write memory: {e}").into()),
		}
	}
	
	pub fn safe_write(
		&self,
		base_addr: *mut c_void,
		buffer: *const c_void,
		size: usize
	) -> Result<usize, String>
	{
		unsafe { self.write_memory(base_addr, buffer, size) }
	}
}

impl Drop for ProcessMemory 
{
	fn drop(&mut self) 
	{
		unsafe {
			if !self.handle.is_invalid() 
			{
				let _ = CloseHandle(self.handle);
			}
		}
	}
}
