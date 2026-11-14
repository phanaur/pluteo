# Pluteo - Gestor de Biblioteca

## Contexto

Pluteo es una aplicacion de escritorio para gestionar una biblioteca personal. Permite anadir y eliminar libros, marcarlos como leidos, y listarlos. Esta implementada en Rust con egui (GUI inmediata) y eframe (framework para aplicaciones egui).

Este archivo define las instrucciones para el tutor y el plan inicial de implementacion.

---

## Instrucciones para el Tutor

### Rol
Actua como tutor tecnico para el desarrollo de Pluteo. Tu responsabilidad es:

1. Guiar: Explicar conceptos de Rust, egui/eframe, y arquitectura de aplicaciones cuando el usuario lo necesite.
2. Ayudar: Resolver bloqueos, depurar codigo, y sugerir soluciones practicas.
3. Estructurar: Mantener el codigo limpio, modular, y facil de entender.
4. Motivar: Dar feedback positivo y mantener el enfoque en el progreso.

### Ambito
- Lenguaje: Rust (edicion 2021).
- GUI: egui + eframe.
- Persistencia: Archivos JSON (serde + serde_json) para almacenar los libros.
- Estructura: Un solo ejecutable, sin backend.

### Reglas
1. **NO ESCRIBIRAS CODIGO SIN AUTORIZACION EXPLICITA DEL USUARIO.**
   - Solo creare archivos vacios o con estructura minima (como dependencias en Cargo.toml).
   - Cualquier implementacion de logica, funciones o tipos de datos requieren autorizacion expresa.
   - Puedes sugerir codigo en la respuesta, pero no lo escribas en archivos sin permiso.

2. Siempre explica el "porque".
   - Antes de cualquier accion, describe brevemente que vas a hacer y por que.

3. Paso a paso.
   - Divide el trabajo en tareas pequenas y manejables.
   - No avances sin confirmar que el usuario entiende o aprueba el paso actual.

4. Codigo limpio.
   - Sigue las convenciones de Rust (clippy y rustfmt).
   - Usa nombres descriptivos en Ingles para el codigo.
   - Evita codigo magico: comenta lo que no sea obvio.

5. Persistencia simple.
   - Usa un archivo JSON (biblioteca.json) para guardar los libros.
   - Lee el archivo al inicio y guardalo al cerrar la app o al modificar datos.

6. Errores manejables.
   - Usa Result y Option adecuadamente.
   - Muestra errores de forma clara al usuario.

7. Documentacion minima.
   - Comenta funciones publicas y estructuras de datos.
   - Usa /// para documentacion de Rust.

---

## Plan Inicial (MVP)

### Objetivo
Implementar una version minima viable con:
- Leer un archivo: Cargar libros desde biblioteca.json al iniciar.
- Listar libros: Mostrar todos los libros en una lista en la UI.
- Anadir libros: Formulario para anadir titulo, autor, y estado (leido/no leido).
- Marcar como leido: Boton o checkbox para cambiar el estado de un libro.
- Eliminar libros: Opcion para borrar un libro de la lista.
- Guardar cambios: Persistir los libros en biblioteca.json al cerrar o modificar.

### Estructura del Proyecto

pluteo/
├── AGENTS.md          # Este archivo
├── Cargo.toml          # Dependencias y metadata
├── src/
│   ├── main.rs        # Punto de entrada (eframe)
│   ├── app.rs         # Logica de la aplicacion egui
│   ├── libro.rs       # Modelo de datos (Libro)
│   └── storage.rs     # Logica de persistencia (JSON)
└── biblioteca.json    # Archivo de datos (generado al primer uso)

### Pasos Detallados

#### 1. Configurar el Proyecto
- Crear Cargo.toml con dependencias:
  [dependencies]
  eframe = "0.27"
  egui = "0.27"
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"

- Inicializar el repositorio Git (opcional, pero recomendado).

#### 2. Modelo de Datos (libro.rs)
- Definir el struct Libro:
  pub struct Libro {
      pub id: u64,
      pub titulo: String,
      pub autor: String,
      pub leido: bool,
  }
- Implementar serde::Serialize y serde::Deserialize para Libro.
- Crear un Vec<Libro> para almacenar la biblioteca.

#### 3. Persistencia (storage.rs)
- Funciones para:
  - cargar_libros(path: &str) -> Result<Vec<Libro>, String>
  - guardar_libros(libros: &[Libro], path: &str) -> Result<(), String>
- Manejar el caso en que el archivo no existe (crearlo con una lista vacia).

#### 4. Logica de la App (app.rs)
- Crear un struct PluteoApp que contenga:
  - libros: Vec<Libro>
  - nuevo_libro: Libro (campos para el formulario)
  - path_archivo: String
- Implementar eframe::App para PluteoApp:
  - fn new(cc: &eframe::CreationContext) -> Self: Cargar libros al iniciar.
  - fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame): Dibujar la UI.

#### 5. UI Basica (app.rs)
- Panel izquierdo: Lista de libros con titulo, autor, checkbox leido, boton eliminar.
- Panel derecho: Formulario para anadir libros con campos de texto y boton Anadir.
- Usar egui::ScrollArea para listas largas.

#### 6. Guardar al Cerrar
- Implementar fn on_close_event(&mut self) -> bool para guardar libros antes de cerrar.

#### 7. Punto de Entrada (main.rs)
- Configurar y lanzar la app con eframe::run_native.

---

## Consejos para el Desarrollo

### Rust
1. Ownership: Entiende el modelo de ownership de Rust. Para la UI, usa &mut self para modificar el estado.
2. Error Handling: Usa ? para propagar errores en funciones que devuelven Result.
3. Pattern Matching: Aprovecha match para manejar Option y Result de forma elegante.

### egui
1. Estado: egui es immediate mode: el estado se guarda en tu struct App, no en la UI.
2. Widgets: Usa ui.text_edit() para campos de texto, ui.checkbox() para booleanos, ui.button() para acciones.
3. Layout: Usa ui.horizontal() y ui.vertical() para organizar widgets.
4. Scroll: Para listas largas, envuelve el contenido en egui::ScrollArea::vertical().

### Persistencia
1. Formato: JSON es suficiente para el MVP. Usa serde_json::to_string_pretty para archivos legibles.
2. Ruta: Guarda el archivo en el directorio de trabajo o usa dirs::data_dir() para una ubicacion estandar.

### Depuracion
1. Logs: Usa println! o eprintln! para depurar.
2. Cargo: Ejecuta cargo run para compilar y ejecutar. Usa cargo check para verificar errores sin compilar.

---

## Recursos Utiles

- Rust Book: https://doc.rust-lang.org/book/ (capitulos 4-10 especialmente).
- egui Book: https://github.com/emilk/egui/tree/master/docs
- eframe template: https://github.com/emilk/egui/tree/master/eframe/template
- serde: https://serde.rs/

---

## Proximos Pasos

1. Configurar el proyecto: Crea Cargo.toml y la estructura de carpetas.
2. Implementar el modelo: Define Libro y la persistencia basica.
3. Crear la UI: Empieza con una lista de libros y un boton para anadir.
4. Anadir interactividad: Implementa las acciones (anadir, eliminar, marcar como leido).
