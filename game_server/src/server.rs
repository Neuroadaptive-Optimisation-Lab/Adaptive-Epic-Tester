use std::mem;
use std::net::{TcpListener, ToSocketAddrs, TcpStream};
use std::io::{Read, Write, Error as IoError};
use std::time::{Duration};

const F64_SIZE : usize = mem::size_of::<f64>();
const COORD_SIZE : usize = mem::size_of::<Coordinate>();

#[derive(Debug)]
pub struct Coordinate
{
	x : f64,
	y : f64,
	z : f64
}

impl Coordinate
{
	pub fn new(x : f64, y : f64, z: f64) -> Coordinate
	{
		Self
		{
			x : x,
			y : y,
			z : z
		}
	}

	pub fn x(&self) -> f64
	{
		self.x
	}

	pub fn y(&self) -> f64
	{
		self.y
	}

	pub fn z(&self) -> f64
	{
		self.z
	}
}

#[derive(Debug)]
struct SolverClient<T>
{
	stream : T
}

impl<T> SolverClient<T>
{
	pub fn new(stream: T) -> Self
	{
		Self
		{
			stream : stream
		}
	}
}

impl SolverClient<TcpStream>
{
	pub fn has_data(&self) -> bool
	{
		let mut buffer = [0; 1];
		match self.stream.peek(&mut buffer)
		{
			Ok(n) => n > 0,
			Err(_) => false
		}
	}
}

impl<T : Write> SolverClient<T>
{
	pub fn send_data(&mut self, data: f64) -> Result<usize, IoError>
	{
		self.stream.write(&data.to_le_bytes())
	}
}

impl<T : Read> SolverClient<T>
{
	pub fn get_coordinate(&mut self) -> Result<Coordinate, IoError>
	{
		let mut buffer : [u8; COORD_SIZE] = [0; COORD_SIZE];
		self.stream.read_exact(&mut buffer)?;
		Self::parse(&buffer)
	}

	fn parse(buffer: &[u8; COORD_SIZE]) -> Result<Coordinate, IoError>
	{
		let x = Self::parse_n(buffer, 0);
		let y = Self::parse_n(buffer, F64_SIZE);
		let z = Self::parse_n(buffer, F64_SIZE * 2);
		Ok(Coordinate::new(x, y, z))
	}

	fn parse_n(buffer: &[u8; COORD_SIZE], offset: usize) -> f64
	{
		let mut parse_buffer : [u8; F64_SIZE] = [0; F64_SIZE];
		for idx in 0..F64_SIZE
		{
			parse_buffer[idx] = buffer[offset + idx];
		}
		f64::from_le_bytes(parse_buffer)
	}
}


#[derive(Debug)]
pub struct GameServer
{
	listener : Option<TcpListener>,
	solver : Option<SolverClient<TcpStream>>,
}

impl GameServer
{
	pub fn new() -> Self
	{
		GameServer
		{
			listener : None,
			solver : None,
		}
	}

	pub fn start<T: ToSocketAddrs>(&mut self, address: T) -> Result<(), IoError>
	{
		let listener = TcpListener::bind(address)?;
    	for stream in listener.incoming()
    	{
        	let _ = self.handle_client(stream?);
        	if self.have_client()
        	{
        		break;
        	}
    	}
    	//hold onto even though not listening to keep hold of socket
		self.listener = Some(listener);
    	Ok(())
	}

	pub fn get_coordinate(&mut self) -> Option<Result<Coordinate, IoError>>
	{
		self.solver.as_mut().map(|s| s.get_coordinate())
	}

	pub fn send_data(&mut self, data : f64) -> Result<usize, IoError>
	{
		match self.solver.as_mut()
		{
			None => Ok(0),
			Some(s) =>  s.send_data(data)
		}
	}

	pub fn has_data(&self) -> bool
	{
		self.solver.as_ref().map(|s| s.has_data()).unwrap_or(false)
	}

	fn handle_client(&mut self, stream : TcpStream) -> Result<(), IoError>
	{
		stream.set_read_timeout(Some(Duration::from_millis(100)))?;
		self.solver = Some(SolverClient::new(stream));
		Ok(())
	}

	fn have_client(&self) -> bool
	{
		self.solver.is_some()
	}
}


#[cfg(test)]
mod solver_tests
{
    use super::*;

    #[test]
    fn can_parse_coordinate()
    {
    	let mut buffer: [u8; COORD_SIZE] = [0; COORD_SIZE];
    	let x = 1.0f64.to_le_bytes();
    	let y = 2.0f64.to_le_bytes();
    	let z = 3.0f64.to_le_bytes();
    	let mut idx = 0;
    	for i in x.iter().chain(y.iter().chain(z.iter()))
    	{
    		buffer[idx] = *i;
    		idx += 1;
    	}
    	let coord = SolverClient::<std::io::Empty>::parse(&buffer).unwrap();
    	assert_eq!(coord.x(),1.0);
    	assert_eq!(coord.y(),2.0);
    	assert_eq!(coord.z(),3.0);
    }
}