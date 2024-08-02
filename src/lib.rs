

pub struct DeviceMetadata {
    id: u32,
    manufacturer: u32,
    model: u32,
    model_version: u32
}

pub struct DeviceManager {
    can_conn: socketcan::CanSocket
}

impl DeviceManager {
    pub fn new(conn: socketcan::CanSocket) -> Self {
        return DeviceManager { can_conn: conn }
    }

    
}