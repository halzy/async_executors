#![ cfg( feature = "async_std" ) ]

// Tested:
//
// ✔ pass a     AsyncStd  to a function that takes exec: `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<AsyncStd> to a function that takes exec: `impl Spawn`
// ✔ pass a     AsyncStd  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&dyn SpawnHandle`
//
mod common;

use
{
	common  :: { *                        } ,
	futures :: { channel::mpsc, StreamExt } ,
};


// pass a AsyncStd to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn test_spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment_ref( 4, &exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, &exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment_clone( 4, &exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a Arc<AsyncStd> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, Arc::new(exec), tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle()
{
	let exec   = AsyncStd::default();
	let result = AsyncStd::block_on( increment_spawn_handle( 4, exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let exec   = AsyncStd::default();
	let result = AsyncStd::block_on( increment_spawn_handle( 4, Arc::new(exec) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let exec   = AsyncStd::default();
	let result = AsyncStd::block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}
