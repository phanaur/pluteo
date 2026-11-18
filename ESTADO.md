# Estado de Pluteo

## Implementado

- Ventana de iced de 400 × 300 con el título «Plúteo».
- Dos libros de ejemplo en memoria. La vista alinea título, autor y estado de
  lectura bajo sus encabezados.
- Casillas de solo lectura que reflejan `is_read` y muestran `Read` o `Unread`.

## Verificación

- Pasaron `cargo fmt --check`, `cargo check --locked --offline` y
  `cargo clippy --locked --offline -- -D warnings`. Cargo avisa de que `serde_json`
  todavía está sin usar.
- `cargo run --locked --offline` compiló el ejecutable. La sesión del agente no
  pudo abrir la ventana por falta de conexión con el compositor Wayland.

## Pendiente

- Completar `Book` con un identificador estable y único y serialización con Serde.
  Es el siguiente paso de [PLAN.md](PLAN.md).
- Cargar y guardar `biblioteca.json`, añadir y eliminar libros, y permitir cambiar
  su estado de lectura al pulsar la casilla.
- Más adelante, ajustar la tipografía, truncar títulos largos con puntos
  suspensivos y permitir redimensionar las columnas con el ratón.
