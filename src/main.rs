mod process_memory;

use process_memory::ProcessMemory;

use std::{
	process::exit,
	ffi::c_void,
	env,
};

pub fn parse_args(args: Vec<String>)
{
	if args.len() < 5
	{
		eprintln!("Usage: {} --pid <number> --address <address> [--read | --write <value>]", args.get(0).unwrap_or(&"program".to_string()));
		exit(1);
	}

	let mut pid: Option<u32> = None;
	let mut base_addr: Option<*const c_void> = None;
	let mut mode: Option<String> = None;
	let mut write_value: Option<u32> = None;

	let mut i: usize = 1; // Skip program name
	while i < args.len()
	{
		match args[i].as_str()
		{
			"--pid" if i + 1 < args.len() => {
				pid = args[i + 1].parse::<u32>().ok();
				i += 2;
			}
			"--address" if i + 1 < args.len() => {
				let raw: &str = args[i + 1].trim_start_matches("0x");
				if let Ok(addr) = u64::from_str_radix(raw, 16).or_else(|_| raw.parse::<u64>())
				{
					base_addr = Some(addr as *const c_void);
				}
				else
				{
					eprintln!("Invalid address format: {}", args[i + 1]);
					exit(1);
				}
				i += 2;
			}
			"--read" => {
				mode = Some("read".to_string());
				i += 1;
			}
			"--write" if i + 1 < args.len() => {
				if let Ok(val) = args[i + 1].parse::<u32>()
				{
					write_value = Some(val);
					mode = Some("write".to_string());
				}
				else
				{
					eprintln!("Invalid write value: {}", args[i + 1]);
					exit(1);
				}
				i += 2;
			}
			_ => {
				eprintln!("Unknown or malformed argument: {}", args[i]);
				exit(1);
			}
		}
	}

	let pid: u32 = match pid
	{
		Some(p) => p,
		None => {
			eprintln!("Missing --pid argument");
			exit(1);
		}
	};

	let base_addr: *const c_void = match base_addr
	{
		Some(addr) => addr,
		None => {
			eprintln!("Missing or invalid --address argument");
			exit(1);
		}
	};

	let process_memory: ProcessMemory = ProcessMemory::safe_new(pid);

	match mode.as_deref()
	{
		Some("read") => {
			let mut buffer: u32 = 0;
			let result: Result<usize, String> = process_memory.safe_read(
				base_addr,
				&mut buffer as *mut _ as *mut c_void,
				size_of::<u32>(),
			);
			match result
			{
				Ok(_) => println!("Read value: {}", buffer),
				Err(e) => eprintln!("Read failed: {:?}", e),
			}
		}
		Some("write") => {
			if let Some(value) = write_value
			{
				let result: Result<usize, String> = process_memory.safe_write(
					base_addr as *mut c_void,
					&value as *const _ as *const c_void,
					size_of::<u32>(),
				);
				match result
				{
					Ok(_) => println!("Wrote value: {}", value),
					Err(e) => eprintln!("Write failed: {:?}", e),
				}
			}
			else
			{
				eprintln!("Missing value for --write");
				exit(1);
			}
		}
		_ => {
			eprintln!("Please specify --read or --write <value>");
			exit(1);
		}
	}
}

fn main()
{
	let mut args: env::Args = env::args();
	if args.len() == 1
	{
		println!("Usage: {} --pid <number> --address <address> [--read | --write <value>]", args.next().unwrap());
		exit(1);
	}
	parse_args(args.collect());
}