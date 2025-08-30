

pub trait Client {
    fn hello_from_server(&self, msg: &str);
}

pub trait Server {

    fn hello_from_client(&self, msg: &str);
    fn login(&self, name: &str, passworld: &str);
}
