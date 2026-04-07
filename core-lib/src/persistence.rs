use tokio::sync::broadcast;
use shared::types::Telemetry;

pub fn start_persistence_actor(mut rx: broadcast::Receiver<Telemetry>) {
    tokio::spawn(async move {
        // TODO : 
        // 1. Acumular datos en un Vec<Telemetry>.
        // 2. Al llegar a ~4MB (180_000 muestras), gestionar el guardado a disco.
        // Cuidado: Clonar o intercambiar el buffer ANTES del guardado asíncrono
        // para no perder las muestras que lleguen durante la escritura.
        
        loop {
            if let Ok(_data) = rx.recv().await {
                
            }
        }
    });
}