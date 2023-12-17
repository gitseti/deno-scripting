use crate::client_pool::ClientPool;
use deno_core::error::{bad_resource, AnyError};
use deno_core::op2;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;
use paho_mqtt::Message;
use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[op2(fast)]
#[smi]
pub fn op_client_pool_new(
    state: Rc<RefCell<OpState>>,
    #[bigint] client_amount: usize,
) -> Result<ResourceId, AnyError> {
    let mut state_ = state.borrow_mut();
    let rid = state_.resource_table.add(ClientPool::new(client_amount));
    Ok(rid)
}

#[op2(async)]
pub async fn op_client_pool_connect(
    state: Rc<RefCell<OpState>>,
    #[smi] rid: ResourceId,
    #[bigint] connects_per_second: usize,
) {
    let client_pool = state
        .borrow_mut()
        .resource_table
        .get::<ClientPool>(rid)
        .unwrap();

    client_pool.connect(connects_per_second).await;
}

#[op2(fast)]
pub fn op_client_pool_publish(
    state: Rc<RefCell<OpState>>,
    #[smi] rid: ResourceId,
    #[string] topic: String,
    #[string] message: String,
) {
    let mut client_pool = state
        .borrow_mut()
        .resource_table
        .get::<ClientPool>(rid)
        .unwrap();

    let message = Message::new(topic, message, 0);
    client_pool.publish_rand(message);
}

impl Resource for ClientPool {
    fn name(&self) -> Cow<str> {
        "client_pool".into()
    }

    fn close(self: Rc<Self>) {
        //TODO
    }
}
