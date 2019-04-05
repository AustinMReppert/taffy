#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

use crate::style::*;
use crate::algo;
use crate::geometry::Size;
use crate::number::Number;
use crate::result::{Layout, Cache, Result};

type MeasureFunc = Box<Fn(Size<Number>) -> Result<Size<f32>>>;

pub(crate) struct InternalNode {
    pub(crate) style: Style,
    pub(crate) parents: Vec<Weak<RefCell<InternalNode>>>,
    pub(crate) children: Vec<Rc<RefCell<InternalNode>>>,
    pub(crate) measure: Option<MeasureFunc>,
    pub(crate) layout_cache: RefCell<Option<Cache>>,
}

pub struct Node(Rc<RefCell<InternalNode>>);

impl Node {
    pub fn new(style: Style, children: Vec<&Node>) -> Node {
        let mut parent = Node(Rc::new(RefCell::new(InternalNode {
            style: style,
            parents: vec![],
            children: vec![],
            measure: None,
            layout_cache: RefCell::new(None),
        })));
        
        for child in children {
            parent.add_child(child);
        }
        
        parent
    }
    
    pub fn add_child(&mut self, child: &Node) {
        child.0.borrow_mut().parents.push(Rc::downgrade(&self.0));
        self.0.borrow_mut().children.push(Rc::clone(&child.0));
    }
    
    // pub fn children(&self) -> Vec<Node> {}
    
    // pub fn mark_dirty(&mut self) {}
    
    // pub fn set_style(&mut self, Style) {}
    
    // pub fn set_children(&mut self, Vec<Node>) {}
    
    // pub fn add_child(&mut self, Node) {}
    
    // pub fn remove_child(&mut self, Node) -> Node {}
    
    // pub fn replace_child(&mut self, Node, u32) -> Node {}
    
    pub fn compute_layout(&self, size: Size<Number>) -> Result<Layout> {
      algo::compute(&self.0.borrow(), size)
    }
}
