use crate::telemetry::Telemetry;

// Enum para elegir estrategia
pub enum Protocol {
    Postcard,
    Protobuf,
    Json
}

pub struct Codec;

impl Codec {
    //Lo que haría el esp32:
    pub fn encode(data: &Telemetry, protocol: Protocol) -> Vec<u8> {
        match protocol {
            Protocol::Postcard => {
                //Es corta porque la librería hace el trabajo pesado
                postcard::to_stdvec_cobs(data).expect("Error crítico serializando Postcard")
            }
            Protocol::Protobuf => {
                // Implementar serialización Protobuf 
                todo!("Implementar encode protobuf")
            }
            Protocol::Json=>{
                //Implementar serialización Json
                 todo!("Implementar encode Json")
            }
        }
    }
    /// Intenta reconstruir el Struct desde Bytes sucios (Lo que hace el PC)
    pub fn decode(data: &[u8], protocol: Protocol) -> Result<Telemetry, String> {
        match protocol {
            Protocol::Postcard => {
                let mut buffer = data.to_vec();
                // El '_ indica un tiempo de vida inferido: dejamos que el compilador gestione la
                // memoria automáticamente ya que Telemetry no mantiene referencias al buffer original.
                match postcard::from_bytes_cobs::<'_, Telemetry>(&mut buffer) {
                    Ok(t) => {
                        Ok(t)
                    }
                    Err(e) => {
                        Err(format!("Error en Postcard: {:?}", e))
                    }
                }
            }
            Protocol::Protobuf => {
                todo!("Implementar decode protobuf")
            }
            Protocol::Json => {
                todo!("Implementar decode json")
            }
        }
    }
}


pub struct SerialBuffer {
    data: Vec<u8>,
}

impl SerialBuffer {
    pub fn new() -> Self {
        Self { data: Vec::with_capacity(1024) }
    }
    pub fn push_bytes(&mut self, new_data: &[u8]) {
        self.data.extend_from_slice(new_data);
    }
    /// Extraer el siguiente paquete completo.
    pub fn try_pop_frame(&mut self) -> Option<Vec<u8>> {
        // Buscamos el delimitador de COBS (0x00)
        if let Some(index) = self.data.iter().position(|&x| x == 0x00) {
            // 1. Extraemos todo LO ANTERIOR al 0x00 (el payload codificado)
            // drain(range) corta esos bytes del vector original y los devuelve
            let frame: Vec<u8> = self.data.drain(0..index).collect();
            // 2. Eliminamos el 0x00 que usamos de separador (está ahora en el índice 0)
            if !self.data.is_empty() {
                self.data.remove(0); 
            }
            return Some(frame);
        }
        // Si no hay 0x00, el paquete está incompleto. Esperamos más datos.
        None
    }
}
