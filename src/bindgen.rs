use
{
	crate                :: { import::*, JoinHandle, SpawnHandle, LocalSpawnHandle  } ,
	wasm_bindgen_futures :: { spawn_local                                           } ,
};


/// We currently only support a global Bindgen threadpool. In principle this is the only supported
/// executor that allows full control. We could expose an interface that allows users to control
/// the lifetime and scope of a Bindgen threadpool.
//
#[ derive( Clone ) ]
//
pub struct Bindgen {}


impl Bindgen
{
	/// Create a new Bindgen threadpool.
	//
	pub fn new() -> Self
	{
		Self{}
	}



	/// Obtain a handle to this executor that can easily be cloned and that implements
	/// Spawn the trait.
	//
	pub fn handle( &self ) -> Bindgen
	{
		self.clone()
	}
}



impl Spawn for Bindgen
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		spawn_local( future );

		Ok(())
	}
}



impl LocalSpawn for Bindgen
{
	fn spawn_local_obj( &mut self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		spawn_local( future );

		Ok(())
	}
}


impl SpawnHandle for Bindgen
{
	fn spawn_handle<T: 'static + Send>( &mut self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		spawn_local( task );

		Ok( rx.into() )
	}
}


impl LocalSpawnHandle for Bindgen
{
	fn spawn_handle_local<T: 'static>( &mut self, fut: impl Future< Output=T > + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		spawn_local( task );

		Ok( rx.into() )
	}
}



impl std::fmt::Debug for Bindgen
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "WASM Bindgen executor" )
	}
}
