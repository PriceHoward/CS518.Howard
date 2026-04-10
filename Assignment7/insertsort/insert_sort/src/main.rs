use std::cell::RefCell;
use std::rc::Rc;

struct ST<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}

impl<S: 'static, A: 'static> ST<S, A> {
    fn pure(a: A) -> ST<S, A> {
        ST {
            run: Box::new(move |s| (a, s)),
        }
    }
    fn lift(f: impl (FnOnce(S) -> (A, S)) + 'static) -> ST<S, A> {
        ST { run: Box::new(f) }
    }
    fn flat_map<B: 'static>(self, f: impl (FnOnce(A) -> ST<S, B>) + 'static) -> ST<S, B> {
        ST {
            run: Box::new(move |s| {
                let (a, s1) = (self.run)(s);
                (f(a).run)(s1)
            }),
        }
    }
}

impl<S, A: Clone> Clone for STArray<S, A> {
    fn clone(&self) -> Self {
        STArray {
            value: Rc::clone(&self.value),
            _marker: std::marker::PhantomData,
        }
    }
}
struct STArray<S, A> {
    value: Rc<RefCell<Vec<A>>>,
    _marker: std::marker::PhantomData<S>,
}

impl<S: 'static, A: Clone + 'static> STArray<S, A> {
    fn from_vec(xs: Vec<A>) -> ST<S, STArray<S, A>> {
        ST::pure(STArray {
            value: Rc::new(RefCell::new(xs)),
            _marker: std::marker::PhantomData,
        })
    }

    fn size(&self) -> ST<S, usize> {
        let value = Rc::clone(&self.value);
        ST::lift(move |s| (value.borrow().len(), s))
    }

    fn write(&self, i: usize, a: A) -> ST<S, ()> {
        let value = Rc::clone(&self.value);
        ST::lift(move |s| {
            value.borrow_mut()[i] = a;
            ((), s)
        })
    }

    fn read(&self, i: usize) -> ST<S, A> {
        let value = Rc::clone(&self.value);
        ST::lift(move |s| (value.borrow()[i].clone(), s))
    }

    fn freeze(&self) -> ST<S, Vec<A>> {
        let value = Rc::clone(&self.value);
        ST::lift(move |s| (value.borrow().clone(), s))
    }
}

fn run_st<A>(program: impl FnOnce() -> ST<(), A>) -> A {
    let (result, _) = (program().run)(());
    result
}

fn insertion_sort_st<S: 'static>(arr: STArray<S, i32>) -> ST<S, ()> {
    arr.size().flat_map(move |n| {
        (1..n).fold(ST::pure(()), move |acc, i| {
            let arr = arr.clone();
            acc.flat_map(move |_| {
                arr.read(i).flat_map(move |index_element| {
                    inner_loop(arr.clone(), i, i, index_element)
                })
            })
        })
    })
}

fn inner_loop<S: 'static>(
    arr: STArray<S, i32>,
    i: usize,
    j: usize,
    index_element: i32
) -> ST<S, ()> {
    if j == 0 {
        arr.write(j, index_element)
    } else {
        let arr_clone = arr.clone();
        arr.read(j - 1).flat_map(move |prev_element| {
            if prev_element > index_element {
                arr_clone
                    .write(j, prev_element)
                    .flat_map(move |_| { inner_loop(arr_clone, i, j - 1, index_element) })
            } else {
                arr_clone.write(j, index_element)
            }
        })
    }
}

fn insertion_sort_pure(xs: &[i32]) -> Vec<i32> {
    if xs.is_empty() {
        return Vec::new();
    }
    run_st(|| {
        STArray::from_vec(xs.to_vec()).flat_map(|arr| {
            let arr_clone = arr.clone();
            insertion_sort_st(arr).flat_map(move |_| arr_clone.freeze())
        })
    })
}

fn main() {
    let arr = vec![99, 18, 22, 6, 75, 5, 4, 11, 89, 1];
    println!("Before the sort: {:?}", arr);
    let sorted = insertion_sort_pure(&arr);
    println!("After the sort:  {:?}", sorted);
}
