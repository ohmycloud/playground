use std::{sync::Arc, vec};
use std::sync::Weak;

pub trait Observer {
    // We use an associated type for the subject.
    type Subject;
    // The observe() method is called by the subject when an update occurs.
    fn observe(&self, subject: &Self::Subject);
}

pub trait Observable {
    // An assocaited type is used for the observer.
    type Observer;
    fn update(&self);
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}

pub struct Subject {
    // We're storing weak pointers to our Observer objects, where the subject is Self.
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.into(),
        }
    }
    pub fn state(&self) -> &str {
        &self.state.as_ref()
    }
}

struct MyObserver {
    name: String,
}

impl MyObserver {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.into()})
    }
}

impl Observer for MyObserver {
    type Subject = Subject;
    fn observe(&self, subject: &Self::Subject) {
        println!(
            "observed subject with state={:?} in {}",
            subject.state,
            self.name
        );
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;
    fn update(&self) {
        self.observers
        .iter()
        .flat_map(|o| o.upgrade())
        .for_each(|o| o.observe(self));
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
        .retain(|f| {
            !f.ptr_eq(&Arc::downgrade(&observer))
        });
    }
}

fn main() {
    let mut subject = Subject::new("some subject state");
    let observer1 = MyObserver::new("observer1");
    let observer2 = MyObserver::new("observer2");

    subject.attach(observer1.clone());
    subject.attach(observer2.clone());

    subject.update();

}