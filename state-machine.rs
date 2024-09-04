pub trait SessionState {}

// We set the default session state to Initial
#[derive(Debug, Default)]
pub struct Session<State: SessionState = Initial> {
    session_id: Uuid,
    // We'll keep a HashMap of aribitrary properties, which might be stored in a database.
    props: HashMap<String, String>,
    phantom: PhantomData<State>,
}

// the result of the initial state transition
pub enum ResumeResult {
    Invalid,
    Anonymous(Session<Anonymous>),
    Authenticated(Session<Authenticated>),
}

impl Session<Initial> {
    /// returns a new session, defaulting to the anonymous state
    pub fn new() -> Session<Anonymous> {
        Session::<Anonymous> {
            session_id: Uuid::new_v4(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }

    /// returns the result of resuming this session from an existing ID.
    phb fn resume_from(session_id: Uuid) -> ResumeResult {
        ResumeResult::Authenticated(
            Session::<Authenticated> {
                session_id,
                props: HashMap::new(),
                phantom: PhantomData,
            }
        )
    }
}

impl Session<Anonymous> {
    pub fn authenticate(
        self,
        username: &str,
        passoword: &str,
    ) -> Result<Session<Authenticated>, Session<Anonymous>> {
        if !username.is_empty() && !passoword.is_empty() {
            Ok(Session::<Authenticated> {
                session_id: self.session_id,
                props: HashMap::new(),
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Session<Authenticated> {
    pub fn update_property(
        &mut self,
        key: &str,
        value: &str,
    ) {
        if let Some(prop) = self.props.get_mut(key) {
            *prop = value.to_string();
        } else {
            self.props.insert(key.into(), value.into());
        }
    }

    pub fn logout(self) -> Session<LoggedOut> {
        Session {
            session_id: Uuid::nil(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Anonymous;
#[derive(Debug, Default)]
pub struct Authenticated;
#[derive(Debug, Default)]
pub struct LoggedOut;

impl SessionState for Initial {}
impl SessionState for Anonymous {}
impl SessionState for Authenticated {}
impl SessionState for LoggedOut {}

fn main() {
    let session = Session::new();
    println!("{:?}", session);

    if let Ok(mut session) = session.authenticate("username", "password") {
        session.update_property("key", "value");
        println!("{:?}", session);

        let session = session.logout();
        println!("{:?}", session);
    }
}