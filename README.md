[Español](#español) | [English](#english)

---

# <a id="español"></a>Interfaz de Control M2 - Arquitectura de Software

El sistema implementa una arquitectura basada en un Cargo Workspace con separación estricta de responsabilidades.

---

## Raíz del Proyecto (Configuración Global)
* **`/Cargo.toml`**: Define el espacio de trabajo (workspace). Orquesta la compilación conjunta de `shared`, `core-lib`, `cli-app` y `tauri-app`, excluyendo `src-firmware` debido a su arquitectura incompatible (Xtensa).
* **`/rust-toolchain.toml`**: Fija la versión del compilador de Rust (ej. 1.77.0) para el entorno de PC. Asegura que todos compilen con las mismas reglas.
* **`/README.md`**: Documentación principal del proyecto y guía de arquitectura.

---

## 1. Subsistema: `/shared` (Contrato de Datos)
Librería agnóstica de hardware compilada sin la librería estándar (`#![no_std]`). Define la comunicación entre el microcontrolador y el ordenador, eliminando errores de serialización. No contiene lógica de ejecución.
* **`Cargo.toml`**: Configuración del crate. Importa la librería `serde` deshabilitando `default-features` para compatibilidad en entornos embebidos.
* **`src/lib.rs`**: Archivo raíz que expone públicamente el módulo de tipos mediante `pub mod types;`.
* **`src/types.rs`**: Contiene las estructuras (`Telemetry`) y enumeraciones (`Command`, `SystemStatus`) exactas que viajan por el puerto serie.

---

## 2. Subsistema: `/core-lib` (Motor Lógico Backend)
Núcleo de procesamiento asíncrono y pesado. Ingesta, distribuye y persiste datos. Actúa de forma totalmente ciega, desconociendo si existe una interfaz gráfica conectada.
* **`Cargo.toml`**: Dependencias pesadas de lógica pura (Tokio para concurrencia asíncrona, Postcard para binarios, Serialport para hardware).
* **`src/lib.rs`**: Expone los tres pilares lógicos del motor (`pipeline`, `ingestion`, `persistence`).
* **`src/pipeline.rs`**: Implementa la Tubería de Datos Interna. Crea un canal Publish/Subscribe (Fan-Out) que permite a múltiples consumidores leer el flujo de datos sin bloquearse entre sí.
* **`src/ingestion.rs`**: Maneja la conexión física por USB. Contiene el hilo bloqueante que lee ráfagas, aplica el algoritmo de enmarcado COBS y deserializa el flujo binario.
* **`src/persistence.rs`**: Actor asíncrono suscrito a la tubería. Acumula el flujo continuo de telemetría en memoria RAM y coordina su volcado binario hacia el disco duro.
* **`tests/test_pipeline.rs`**: Pruebas de integración automatizadas. Valida que la inyección de paquetes en la ingesta fluye correctamente a través del pipeline asíncrono.

---

## 3. Subsistema: `/cli-app` (Interfaz de Terminal)
Ejecutable de terminal que importa el núcleo lógico. Sirve para operaciones de diagnóstico, pruebas de estrés y desarrollo temprano sin la sobrecarga del motor web.
* **`Cargo.toml`**: Configuración básica del ejecutable CLI.
* **`src/main.rs`**: Punto de entrada de consola. Instancia el `SystemPipeline`, permitiendo monitorizar el flujo de datos crudos e inyectar comandos directamente desde la terminal.

---

## 4. Subsistema: `/tauri-app` (Interfaz Gráfica Principal)
Envoltorio visual interactivo. Conecta el backend pesado en Rust con un frontend ligero renderizado en un WebView a través de un puente de comunicación de bajo coste (IPC).

### Frontend (Vista Web)
* **`package.json`, `package-lock.json`**: Gestor de dependencias de Node.js.
* **`astro.config.mjs`, `svelte.config.js`, `tsconfig.json`**: Configuración de los compiladores de UI. Astro actúa como shell, Svelte maneja la reactividad de los componentes.
* **`src/lib/hardware.ts`**: Lógica de cliente. Se encarga de invocar comandos hacia Rust y suscribirse al evento IPC de datos para actualizar la UI.
* **`src/`**: Resto de interfaz.

### Backend (`src-tauri`)
* **`Cargo.toml`, `Cargo.lock`**: Configuración y dependencias del ejecutable puente de Tauri.
* **`build.rs`**: Script pre-compilación requerido por Tauri.
* **`tauri.conf.json`**: Configuración estructural de la ventana, empaquetado y puente IPC.
* **`capabilities/default.json`**: Lista blanca de seguridad que define qué comandos y eventos puede utilizar el frontend.
* **`icons/*`**: Activos gráficos compilados en el binario final.
* **`src/main.rs`**: Punto de entrada mínimo del SO; delega la ejecución a la librería para compatibilidad multiplataforma.
* **`src/lib.rs`**: Orquestador principal. Inicia el motor lógico, define los comandos accesibles por la UI y contiene el actor que implementa la estrategia de chunking (empaquetado de muestras a 60 Hz para evitar la saturación del puente IPC).

---

## 5. Subsistema: `/src-firmware` (Cliente de Hardware ESP32)
Código embebido en Rust nativo (`std` sobre ESP-IDF). Actúa como un cliente ligero sin conversiones matemáticas complejas; su única misión es extraer el dato eléctrico de los buses físicos, aplicar normas de seguridad, empaquetarlo y transmitirlo por USB.
* **`Cargo.toml`**: Gestor del crate embebido. Depende de las capas de abstracción de Espressif (`esp-idf-svc`, `esp-idf-hal`).
* **`rust-toolchain.toml`**: Obliga al uso del compilador específico bifurcado por Espressif para la arquitectura Xtensa.
* **`build.rs`**: Vincula las librerías precompiladas de C (ESP-IDF) con el código en Rust.
* **`sdkconfig.defaults`**: Parámetros de configuración del sistema operativo en tiempo real subyacente (FreeRTOS).
* **`memory.x`**: Define la disposición de la memoria flash y RAM del microcontrolador.
* **`src/main.rs`**: Arquitectura de concurrencia principal. Gestiona la instanciación de hilos y canales de memoria. Asigna el Núcleo 0 a la lectura, el Núcleo 1 al transporte UART y la aplicación del protocolo COBS, y lanza un hilo paralelo simulando un PLC industrial para los lazos de seguridad críticos.
* **`src/hardware.rs`**: Contiene la lógica cruda de interacción electrónica. Abstracción directa sobre los buses físicos (I2C, SPI, ADC interno) y rutinas de calibración.

<br><br>

---

# <a id="english"></a>M2 Control Interface - Software Architecture

The system implements an architecture based on a Cargo Workspace with a strict separation of concerns.

---

## Project Root (Global Configuration)
* **`/Cargo.toml`**: Defines the workspace. Orchestrates the joint compilation of `shared`, `core-lib`, `cli-app`, and `tauri-app`, excluding `src-firmware` due to its incompatible architecture (Xtensa).
* **`/rust-toolchain.toml`**: Pins the Rust compiler version (e.g., 1.77.0) for the PC environment. Ensures everyone compiles with the same rules.
* **`/README.md`**: Main project documentation and architecture guide.

---

## 1. Subsystem: `/shared` (Data Contract)
Hardware-agnostic library compiled without the standard library (`#![no_std]`). Defines the communication between the microcontroller and the computer, eliminating serialization errors. Does not contain execution logic.
* **`Cargo.toml`**: Crate configuration. Imports the `serde` library disabling `default-features` for compatibility in embedded environments.
* **`src/lib.rs`**: Root file that publicly exposes the types module via `pub mod types;`.
* **`src/types.rs`**: Contains the exact structures (`Telemetry`) and enumerations (`Command`, `SystemStatus`) that travel through the serial port.

---

## 2. Subsystem: `/core-lib` (Backend Logic Engine)
Asynchronous and heavy processing core. Ingests, distributes, and persists data. Acts completely blind, unaware if a graphical interface is connected.
* **`Cargo.toml`**: Heavy dependencies of pure logic (Tokio for asynchronous concurrency, Postcard for binaries, Serialport for hardware).
* **`src/lib.rs`**: Exposes the three logical pillars of the engine (`pipeline`, `ingestion`, `persistence`).
* **`src/pipeline.rs`**: Implements the Internal Data Pipeline. Creates a Publish/Subscribe (Fan-Out) channel that allows multiple consumers to read the data flow without blocking each other.
* **`src/ingestion.rs`**: Manages the physical USB connection. Contains the blocking thread that reads bursts, applies the COBS continuous framing algorithm, and deserializes the binary flow.
* **`src/persistence.rs`**: Asynchronous actor subscribed to the pipeline. Accumulates the continuous telemetry flow in RAM and coordinates its binary dump to the hard drive.
* **`tests/test_pipeline.rs`**: Automated integration tests. Validates that the injection of packets in the ingestion flows correctly through the asynchronous pipeline.

---

## 3. Subsystem: `/cli-app` (Terminal Interface)
Terminal executable that imports the logical core. Used for diagnostic operations, stress testing, and early development without the overhead of the web engine.
* **`Cargo.toml`**: Basic CLI executable configuration.
* **`src/main.rs`**: Console entry point. Instantiates the `SystemPipeline`, allowing monitoring of raw data flow and injecting commands directly from the terminal.

---

## 4. Subsystem: `/tauri-app` (Main Graphical Interface)
Interactive visual wrapper. Connects the heavy Rust backend with a lightweight frontend rendered in a WebView through a low-cost Inter-Process Communication (IPC) bridge.

### Frontend (Web View)
* **`package.json`, `package-lock.json`**: Node.js dependency manager.
* **`astro.config.mjs`, `svelte.config.js`, `tsconfig.json`**: UI compiler configurations. Astro acts as a shell, Svelte handles component reactivity.
* **`src/lib/hardware.ts`**: Client logic. Responsible for invoking commands to Rust and subscribing to the data IPC event to update the UI.
* **`src/`**: Rest of the interface.

### Backend (`src-tauri`)
* **`Cargo.toml`, `Cargo.lock`**: Configuration and dependencies of the Tauri bridge executable.
* **`build.rs`**: Pre-compilation script required by Tauri.
* **`tauri.conf.json`**: Structural configuration of the window, packaging, and IPC bridge.
* **`capabilities/default.json`**: Security allowlist that defines which commands and events the frontend can use.
* **`icons/*`**: Graphical assets compiled into the final binary.
* **`src/main.rs`**: Minimal OS entry point; delegates execution to the library for cross-platform compatibility.
* **`src/lib.rs`**: Main orchestrator. Starts the logic engine, defines the commands accessible by the UI, and contains the actor that implements the chunking strategy (packing samples at 60 Hz to avoid saturating the IPC bridge).

---

## 5. Subsystem: `/src-firmware` (ESP32 Hardware Client)
Embedded code in native Rust (`std` on ESP-IDF). Acts as a lightweight client without complex mathematical conversions; its sole mission is to extract the electrical data from the physical buses, apply safety rules, package it, and transmit it via USB.
* **`Cargo.toml`**: Embedded crate manager. Depends on Espressif's abstraction layers (`esp-idf-svc`, `esp-idf-hal`).
* **`rust-toolchain.toml`**: Forces the use of the specific compiler forked by Espressif for the Xtensa architecture.
* **`build.rs`**: Links the precompiled C libraries (ESP-IDF) with the Rust code.
* **`sdkconfig.defaults`**: Configuration parameters of the underlying real-time operating system (FreeRTOS).
* **`memory.x`**: Defines the layout of the microcontroller's flash memory and RAM.
* **`src/main.rs`**: Main concurrency architecture. Manages the instantiation of threads and memory channels. Assigns Core 0 to reading, Core 1 to UART transport and COBS protocol application, and launches a parallel thread simulating an industrial PLC for critical safety loops.
* **`src/hardware.rs`**: Contains the raw electronic interaction logic. Direct abstraction over the physical buses (I2C, SPI, internal ADC) and calibration routines.