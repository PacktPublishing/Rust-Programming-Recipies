use d6_doodle::models::User;
use std::collections::HashMap;
use std::sync::{Mutex,Arc};

pub struct Session(Arc<Mutex<HashMap<u64,User>>>);

impl Session{
    pub fn new()->Self{
        Session(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn get(&self,k:u64)->Option<User>{
        self.0.lock().unwrap().get(&k).map(|u| u.clone())
    }

    pub fn put(&self, u:User)->u64{
        let mut map = self.0.lock().unwrap();
        loop {
            let id = rand::random::<u64>();
            if map.contains_key(&id) {
                continue;
            }
            map.insert(id,u);
            return id;

        }
    }
}
