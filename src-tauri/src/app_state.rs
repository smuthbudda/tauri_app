
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct AppState {
    pub servers: Vec<ServerDetails>,
    pub names: Vec<String>
}
impl AppState {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            names: Vec::new()
        }
    }

    ///  Adds an address to the list of servers
    pub fn add_server(&mut self, server_details: ServerDetails) {
        self.servers.push(server_details);
    }

    pub fn add_name(&mut self, name: String){
        self.names.push(name);
    }
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerDetails{
    pub address: String,
    pub port: String,
    pub name: String
}

impl ServerDetails {
    pub fn new(address: String, port: String, name: String) -> Self {
        Self {
            address,
            port,
            name
        }
    }
}