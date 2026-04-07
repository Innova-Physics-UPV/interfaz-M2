import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export interface Telemetry {
    timestamp: number;
    voltage_kv: number;
    pressure_mbar: number;
    anode_temp_c: number;
    status: string;
}

export async function startPwm() {
    await invoke('send_command', { command: { type: 'StartPwm' } });
}
// TODO: Añadir funciones para StopSafe, Calibrate, etc.