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
                use prost::Message;
                use crate::proto::innova::Telemetry as PbTelemetry;


                let pb = PbTelemetry {
                    timestamp: data.timestamp,
                    pressure_mbar: data.pressure_mbar,
                    voltage_kv: data.voltage_kv,
                    heater_current_a: data.heater_current_a,
                    interlock_engaged: data.interlock_engaged,
                    status: match data.status {
                        crate::telemetry::SystemStatus::Idle => crate::proto::innova::SystemStatus::Idle as i32,
                        crate::telemetry::SystemStatus::Pumping => crate::proto::innova::SystemStatus::Pumping as i32,
                        crate::telemetry::SystemStatus::Preheat => crate::proto::innova::SystemStatus::Preheat as i32,
                        crate::telemetry::SystemStatus::HvOn => crate::proto::innova::SystemStatus::Hvon as i32,
                        crate::telemetry::SystemStatus::Error => crate::proto::innova::SystemStatus::Error as i32,
                    },

                };

                let mut buf = Vec::new();
                pb.encode(&mut buf).expect("Error serializando Protobuf");

                // Añadimos prefijo de longitud (varint)
                let mut framed = Vec::new();
                encode_varint(buf.len() as u64, &mut framed);
                framed.extend_from_slice(&buf);

                framed
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
                use prost::Message;
                use crate::proto::innova::Telemetry as PbTelemetry;

                let pb = PbTelemetry::decode(data)
                .map_err(|e| format!("Error decodificando Protobuf: {}", e))?;

                Ok(crate::telemetry::Telemetry {
                    timestamp: pb.timestamp,
                    pressure_mbar: pb.pressure_mbar,
                    voltage_kv: pb.voltage_kv,
                    heater_current_a: pb.heater_current_a,
                    interlock_engaged: pb.interlock_engaged,
                    status: match pb.status {
                        0 => crate::telemetry::SystemStatus::Idle,
                        1 => crate::telemetry::SystemStatus::Pumping,
                        2 => crate::telemetry::SystemStatus::Preheat,
                        3 => crate::telemetry::SystemStatus::HvOn,
                        _ => crate::telemetry::SystemStatus::Error,
                    },
                })
            }

            Protocol::Json => {
                todo!("Implementar decode json")
            }
        }
    }
}

fn encode_varint(mut v: u64, out: &mut Vec<u8>) {
    while v >= 0x80 {
        out.push(((v as u8) & 0x7F) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
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
    pub fn try_pop_frame_protobuf(&mut self) -> Option<Vec<u8>> {
        if self.data.is_empty() {
            return None;
        }

        let mut len: usize = 0;
        let mut shift = 0usize;
        let mut header_len = 0usize;

        for &byte in &self.data {
            let b = byte as usize;
            len |= (b & 0x7F) << shift;
            header_len += 1;

            if (b & 0x80) == 0 {
                break;
            }

            shift += 7;
            if shift > 28 {
                return None;
            }
        }
        if header_len == self.data.len() && (self.data[header_len - 1] & 0x80) != 0 {
            return None;
        }
        if self.data.len() < header_len + len {
            return None;
        }
        self.data.drain(0..header_len);

        let frame: Vec<u8> = self.data.drain(0..len).collect();
        Some(frame)
    }

}
