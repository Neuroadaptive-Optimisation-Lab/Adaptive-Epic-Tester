extern crate libc;

mod server;

use server::{GameServer};

pub const DEFAULT_PORT : u16 = 34892;

#[no_mangle]
pub extern fn server_new() -> *mut GameServer
{
	Box::into_raw(Box::new(GameServer::new()))
}

#[no_mangle]
pub extern fn server_free(game_server_ptr: *mut GameServer) -> ()
{
	unsafe {Box::from_raw(game_server_ptr);}
}


#[no_mangle]
pub extern fn server_start(game_server_ptr: *mut GameServer, port : u16) -> i32
{
	let server : &mut GameServer = unsafe{&mut *game_server_ptr};
	let address = format!("127.0.0.1:{}", port);
	match server.start(address)
	{
		Ok(_) => 1,
		Err(e) =>
		{
			//only a toy project just print for now
			eprintln!("Error: {:?}", e);
			0
		}
	}
}

#[no_mangle]
pub extern fn server_get_coord(game_server_ptr: *mut GameServer, x : *mut f64, y: *mut f64, z: *mut f64) -> i32
{
	let server : &mut GameServer = unsafe{&mut *game_server_ptr};
	match server.get_coordinate()
	{
		None => 0,
		Some(r) =>
		{
			match r
			{
				Err(e) =>
				{
					eprintln!("e: {:?}", e);
					0
				}
				Ok(c) =>
				{
					unsafe
					{
						*x = c.x();
						*y = c.y();
						*z = c.z();
					}
					1
				}
			}
		}
	}
}



#[no_mangle]
pub extern fn server_send_data(game_server_ptr: *mut GameServer, data : f64) -> i32
{
	let server : &mut GameServer = unsafe{&mut *game_server_ptr};
	match server.send_data(data)
	{

		Err(e) =>
		{
			eprintln!("e: {:?}", e);
			0
		}
		Ok(_) =>
		{
			1
		}
	}
}


#[no_mangle]
pub extern fn server_has_data(game_server_ptr: *const GameServer) -> i32
{
	let server : &GameServer = unsafe{&*game_server_ptr};
	if server.has_data() {1} else {0}
}
