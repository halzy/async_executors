use
{
	crate          :: { SpawnHandle, JoinHandle      } ,
	tokio::runtime :: { Handle as TokioRtHandle      } ,
	futures_task   :: { FutureObj, Spawn, SpawnError } ,
	futures_util   :: { future::abortable            } ,
	std            :: { sync::atomic::AtomicBool     } ,

};

/// A handle to a [TokioCt](crate::TokioCt) or [TokioTp](crate::TokioTp) executor. It implements `Spawn` and `SpawnHandle` traits.
/// You can obtain one from [TokioCt::handle](crate::TokioCt::handle) or [TokioTp::handle](crate::TokioTp::handle).
///
/// For [TokioTp](crate::TokioTp) this can be used to avoid a drop order problem for the tokio Runtime. See the
/// documentation for [TokioTp](crate::TokioTp) for an explanation.
///
/// For [TokioCt](crate::TokioCt) this can be used to send a future from another thread to run on the [TokioCt](crate::TokioCt).
///
/// The handle is only operational as long as the parent executor is alive. There is no compiler
/// assisted lifetime tracking for this as generally spawned futures you would like to give the
/// handle to need to be `'static`, so usability would be rather hampered. You must make
/// sure you manage the lifetimes manually.
///
/// If the parent executor is already dropped when [spawn](futures_util::task::SpawnExt::spawn) is called, the future just
/// get's dropped silently without ever being polled.
///
/// ## Unwind Safety.
///
/// You must only spawn futures to this API that are unwind safe. Tokio will wrap it in
/// [std::panic::AssertUnwindSafe] and wrap the poll invocation with [std::panic::catch_unwind].
///
/// They reason that this is fine because they require `Send + 'static` on the future. As far
/// as I can tell this is wrong. Unwind safety can be circumvented in several ways even with
/// `Send + 'static` (eg. `parking_lot::Mutex` is `Send + 'static` but `!UnwindSafe`).
///
/// You should make sure that if your future panics, no code that lives on after the spawned task has
/// unwound, nor any destructors called during the unwind can observe data in an inconsistent state.
///
/// See the relevant [catch_unwind RFC](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md)
/// and it's discussion threads for more info as well as the documentation of [std::panic::UnwindSafe].
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioHandle
{
	pub(crate) spawner: TokioRtHandle,
}



impl TokioHandle
{
	pub(crate) fn new( spawner: TokioRtHandle ) -> Self
	{
		Self { spawner }
	}
}



impl Spawn for TokioHandle
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.spawner.spawn( future );

		Ok(())
	}
}



impl<Out: 'static + Send> SpawnHandle<Out> for TokioHandle
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, a_handle) = abortable( future );

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::Tokio
		{
			handle  : self.spawner.spawn( fut ) ,
			detached: AtomicBool::new( false )  ,
			a_handle                            ,
		}})
	}
}
