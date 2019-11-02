#![ cfg( feature = "bindgen" )]

// Tested:
//
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl Spawn`
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawn`
// ✔ pass a      Bindgen to a function that takes exec: `impl Spawn      + Clone`
// ✔ pass a      Bindgen to a function that takes exec: `impl LocalSpawn + Clone`
//
mod common;

use
{
	common            :: { *                        } ,
	async_executors   :: { *                        } ,
	wasm_bindgen_test :: { *                        } ,
	futures           :: { StreamExt, channel::mpsc } ,
};

wasm_bindgen_test_configure!(run_in_browser);

// pass a &mut Bindgen to a function that takes exec: `&mut impl Spawn`
//
#[wasm_bindgen_test]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment( 4, &mut exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawn`
//
#[wasm_bindgen_test]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment_local( 4, &mut exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `impl Spawn + Clone`
//
#[wasm_bindgen_test]
//
fn test_spawn_from_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment_by_value( 4, exec.handle(), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `impl LocalSpawn + Clone`
//
#[wasm_bindgen_test]
//
fn test_spawn_from_handle_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment_by_value_local( 4, exec.handle(), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}