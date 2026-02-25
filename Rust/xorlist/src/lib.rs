struct Node<T> {
    value: T,
    both: Option<*mut Node<T>>,
}

pub struct XorList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    list_length: usize,
}

impl<T> XorList<T> {
    pub fn new() -> Self {
        XorList {
            head: None,
            tail: None,
            list_length: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Box::into_raw(
            Box::new(Node {
                value: val,
                both: self.head,
            })
        );
        if self.list_length == 0 {
            self.tail = Some(new_node);
        } else {
            if let Some(prev_head) = self.head {
                unsafe {
                    (*prev_head).both = Some(
                        ((new_node as usize) ^
                            (match (*prev_head).both {
                                Some(var) => var as usize,
                                None => 0,
                            })) as *mut Node<T>
                    );
                }
            }
        }
        self.head = Some(new_node);
        self.list_length += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Box::into_raw(
            Box::new(Node {
                value: val,
                both: self.tail,
            })
        );

        if self.list_length == 0 {
            self.head = Some(new_node);
        } else {
            if let Some(prev_tail) = self.tail {
                unsafe {
                    (*prev_tail).both = Some(
                        ((
                            (match (*prev_tail).both {
                                Some(var) => var as usize,
                                None => 0,
                            }) as usize
                        ) ^ (new_node as usize)) as *mut Node<T>
                    );
                }
            }
        }
        self.tail = Some(new_node);
        self.list_length += 1;
    }

    pub fn insert(&mut self, val: T, index: usize) {
        if index == 0 {
            self.push_front(val);
        } else if index == self.list_length {
            self.push_back(val);
        } else {
            let mut prev: Option<*mut Node<T>> = None;
            let mut curr: Option<*mut Node<T>> = self.head;
            for _ in 0..index {
                if let Some(curr_ptr) = curr {
                    unsafe {
                        let prev_addr = match prev {
                            Some(p) => p as usize,
                            None => 0,
                        };

                        let both_addr: usize = match (*curr_ptr).both {
                            Some(b) => b as usize,
                            None => 0,
                        };

                        let next = if (prev_addr ^ both_addr) == 0 {
                            None
                        } else {
                            Some((prev_addr ^ both_addr) as *mut Node<T>)
                        };
                        prev = curr;
                        curr = next;
                    }
                }
            }
            unsafe {
                if let Some(curr_ptr) = curr {
                    let prev_addr = match prev {
                        Some(p) => p as usize,
                        None => 0,
                    };

                    let both_addr = match (*curr_ptr).both {
                        Some(b) => b as usize,
                        None => 0,
                    };
                    let _next = if
                        (prev_addr ^ both_addr) == 0 //_ Suppresses warning of Unused Variable.
                    {
                        None
                    } else {
                        Some((prev_addr ^ both_addr) as *mut Node<T>)
                    };

                    let new_node = Box::into_raw(
                        Box::new(Node {
                            value: val,
                            both: Some((prev_addr ^ (curr_ptr as usize)) as *mut Node<T>),
                        })
                    );

                    if let Some(prev_ptr) = prev {
                        (*prev_ptr).both = Some(
                            ((match (*prev_ptr).both {
                                Some(pp) => pp as usize,
                                None => 0,
                            }) ^
                                (curr_ptr as usize) ^
                                (new_node as usize)) as *mut Node<T>
                        );
                    }

                    if let Some(curr_ptr) = curr {
                        (*curr_ptr).both = Some(
                            ((match (*curr_ptr).both {
                                Some(cp) => cp as usize,
                                None => 0,
                            }) ^
                                prev_addr ^
                                (new_node as usize)) as *mut Node<T>
                        );
                    }
                }
                self.list_length += 1;
            }
        }
    }

    pub fn delete(&mut self, index: usize) {
        let mut prev: Option<*mut Node<T>> = None;
        let mut curr: Option<*mut Node<T>> = self.head;
        for _ in 0..index {
            if let Some(curr_ptr) = curr {
                unsafe {
                    let prev_addr = match prev {
                        Some(p) => p as usize,
                        None => 0,
                    };

                    let both_addr = match (*curr_ptr).both {
                        Some(b) => b as usize,
                        None => 0,
                    };

                    let next = if (prev_addr ^ both_addr) == 0 {
                        None
                    } else {
                        Some((prev_addr ^ both_addr) as *mut Node<T>)
                    };
                    prev = curr;
                    curr = next;
                }
            }
        }
        if let Some(curr_ptr) = curr {
            unsafe {
                let prev_addr = match prev {
                    Some(p) => p as usize,
                    None => 0,
                };

                let both_addr = match (*curr_ptr).both {
                    Some(b) => b as usize,
                    None => 0,
                };
                let next = if (prev_addr ^ both_addr) == 0 {
                    None
                } else {
                    Some((prev_addr ^ both_addr) as *mut Node<T>)
                };
                if let Some(prev_ptr) = prev {
                    let new_both =
                        (match (*prev_ptr).both {
                            Some(pp) => pp as usize,
                            None => 0,
                        }) ^
                        (curr_ptr as usize) ^
                        (match next {
                            Some(n) => n as usize,
                            None => 0,
                        });
                    (*prev_ptr).both = if new_both == 0 {
                        None
                    } else {
                        Some(new_both as *mut Node<T>)
                    };
                }
                if let Some(next_ptr) = next {
                    let new_both =
                        (match (*next_ptr).both {
                            Some(np) => np as usize,
                            None => 0,
                        }) ^
                        (curr_ptr as usize) ^
                        (match prev {
                            Some(p) => p as usize,
                            None => 0,
                        });
                    (*next_ptr).both = if new_both == 0 {
                        None
                    } else {
                        Some(new_both as *mut Node<T>)
                    };
                }
                if prev.is_none() {
                    self.head = next;
                }
                if next.is_none() {
                    self.tail = prev;
                }
                drop(Box::from_raw(curr_ptr));
                self.list_length -= 1;
            }
        }
    }

    pub fn traverse<F: Fn(&T)>(&self, f: F) {
        let mut prev: Option<*mut Node<T>> = None;
        let mut curr: Option<*mut Node<T>> = self.head;
        while let Some(curr_ptr) = curr {
            unsafe {
                f(&(*curr_ptr).value);
                let prev_addr = match prev {
                    Some(p) => p as usize,
                    None => 0,
                };

                let both_addr = match (*curr_ptr).both {
                    Some(b) => b as usize,
                    None => 0,
                };

                let next = if (prev_addr ^ both_addr) == 0 {
                    None
                } else {
                    Some((prev_addr ^ both_addr) as *mut Node<T>)
                };
                prev = curr;
                curr = next;
            }
        }
    }
}

impl<T> Drop for XorList<T> {
    fn drop(&mut self) {
        let mut prev: Option<*mut Node<T>> = None;
        let mut curr = self.head;
        while let Some(curr_ptr) = curr {
            unsafe {
                let prev_addr = match prev {
                    Some(p) => p as usize,
                    None => 0,
                };
                let both_addr = match (*curr_ptr).both {
                    Some(b) => b as usize,
                    None => 0,
                };
                let next = if (prev_addr ^ both_addr) == 0 {
                    None
                } else {
                    Some((prev_addr ^ both_addr) as *mut Node<T>)
                };
                prev = curr;
                drop(Box::from_raw(curr_ptr));
                curr = next;
            }
        }
    }
}
