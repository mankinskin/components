use seed::{
    prelude::*,
};
use rql::{
    Id,
};
use crate::{
    Component,
    Viewable,
    editor::{
        Edit,
    },
};
use database_table::{
    TableItem,
};
use std::result::Result;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Model<T> {
    pub data: T,
}
impl<T: Default> Default for Model<T> {
    fn default() -> Self {
        Self::from(T::default())
    }
}
impl<T> From<T> for Model<T> {
    fn from(data: T) -> Self {
        Self {
            data,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Msg<T: Component + TableItem> {
    Post,
    Posted(Result<Id<T>, String>),
    Data(<T as Component>::Msg),
}
impl<T: Component + TableItem + Debug> Component for Model<T> {
    type Msg = Msg<T>;
    fn update(&mut self, msg: Self::Msg, orders: &mut impl Orders<Self::Msg>) {
        match msg {
            Msg::Post => {
                //orders.perform_cmd(
                //    T::post(self.data.clone()).map(|res| Msg::Posted(res))
                //);
            },
            Msg::Posted(res) => {
                match res {
                    Ok(id) => { seed::log(id); },
                    Err(e) => { seed::log(e); },
                }
            },
            Msg::Data(msg) => {
                self.data.update(msg, &mut orders.proxy(Msg::Data))
            },
        }
    }
}
impl<T: Viewable + TableItem + Debug> Viewable for Model<T> {
    fn view(&self) -> Node<Self::Msg> {
        self.data.view().map_msg(Msg::Data)
    }
}
impl<T: Edit + TableItem + Debug> Edit for Model<T> {
    fn edit(&self) -> Node<Self::Msg> {
        self.data.edit().map_msg(Msg::Data)
    }
}
