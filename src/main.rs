extern crate oasis_runtime_api;
extern crate failure;
extern crate io_context;
extern crate oasis_core_runtime;

use std::sync::Arc;

use failure::{format_err, Fallible};
use io_context::Context as IoContext;

use oasis_runtime_api::{with_api, KeyValue};
use oasis_core_runtime::{
    common::{runtime::RuntimeId, version::Version},
    rak::RAK,
    register_runtime_txn_methods, runtime_context,
    storage::StorageContext,
    transaction::{dispatcher::CheckOnlySuccess, Context as TxnContext},
    version_from_cargo, Protocol, RpcDemux, RpcDispatcher, TxnDispatcher, TxnMethDispatcher,
};

struct Context {
    test_runtime_id: RuntimeId,
}

/// Return previously set runtime ID of this runtime.
fn get_runtime_id(_args: &(), ctx: &mut TxnContext) -> Fallible<Option<String>> {
    let rctx = runtime_context!(ctx, Context);

    Ok(Some(rctx.test_runtime_id.to_string()))
}

/// Insert a key/value pair.
fn insert(args: &KeyValue, ctx: &mut TxnContext) -> Fallible<Option<String>> {
    if args.value.as_bytes().len() > 128 {
        return Err(format_err!("Value too big to be inserted."));
    }
    if ctx.check_only {
        return Err(CheckOnlySuccess::default().into());
    }
    ctx.emit_txn_tag(b"kv_op", b"insert");
    ctx.emit_txn_tag(b"kv_key", args.key.as_bytes());

    let existing = StorageContext::with_current(|mkvs, _untrusted_local| {
        mkvs.insert(
            IoContext::create_child(&ctx.io_ctx),
            args.key.as_bytes(),
            args.value.as_bytes(),
        )
    });
    Ok(existing.map(String::from_utf8).transpose()?)
}

/// Retrieve a key/value pair.
fn get(args: &str, ctx: &mut TxnContext) -> Fallible<Option<String>> {
    if ctx.check_only {
        return Err(CheckOnlySuccess::default().into());
    }
    ctx.emit_txn_tag(b"kv_op", b"get");
    ctx.emit_txn_tag(b"kv_key", args.as_bytes());

    let existing = StorageContext::with_current(|mkvs, _untrusted_local| {
        mkvs.get(IoContext::create_child(&ctx.io_ctx), args.as_bytes())
    });
    Ok(existing.map(String::from_utf8).transpose()?)
}

/// Remove a key/value pair.
fn remove(args: &str, ctx: &mut TxnContext) -> Fallible<Option<String>> {
    if ctx.check_only {
        return Err(CheckOnlySuccess::default().into());
    }
    ctx.emit_txn_tag(b"kv_op", b"remove");
    ctx.emit_txn_tag(b"kv_key", args.as_bytes());

    let existing = StorageContext::with_current(|mkvs, _untrusted_local| {
        mkvs.remove(IoContext::create_child(&ctx.io_ctx), args.as_bytes())
    });
    Ok(existing.map(String::from_utf8).transpose()?)
}

fn main() {
    // Initializer.
    let init = |protocol: &Arc<Protocol>,
                _rak: &Arc<RAK>,
                _rpc_demux: &mut RpcDemux,
                _rpc: &mut RpcDispatcher|
     -> Option<Box<dyn TxnDispatcher>> {
        let mut txn = TxnMethDispatcher::new();

        with_api! { register_runtime_txn_methods!(txn, api); }

        let rt_id = protocol.get_runtime_id();

        txn.set_context_initializer(move |ctx: &mut TxnContext| {
            ctx.runtime = Box::new(Context {
                test_runtime_id: rt_id,
            })
        });

        Some(Box::new(txn))
    };

    // Start the runtime.
    oasis_core_runtime::start_runtime(Box::new(init), version_from_cargo!());
}
