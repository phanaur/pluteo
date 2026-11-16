# Plan incremental de Pluteo

Implementar cada paso cuando el usuario lo autorice. Priorizar un prototipo funcional
antes de introducir módulos o abstracciones adicionales.

1. Ventana inicial con iced: título «Pluteo», tamaño inicial de 400 × 300 y encabezado «Pluteo».
2. Modelo `Book`: identificador estable y único, título, autor y estado de lectura.
   Usar nombres en inglés y serialización con serde.
3. Cargar `biblioteca.json` y mostrar la lista de libros con desplazamiento.
   Distinguir un archivo inexistente de uno corrupto o inaccesible.
4. Formulario para añadir libros, validando título y autor y asignando identificadores sin colisiones.
5. Cambiar el estado de lectura y eliminar libros mediante mensajes de iced.
6. Guardar tras cada modificación confirmada, mostrando los errores y protegiendo el archivo anterior.
   Completar esta persistencia junto con la primera acción que modifique datos.
7. Revisar el flujo completo y refactorizar si facilita entender o probar el código.

## Organización posible

Por ahora basta `src/main.rs`. Si el código crece, se podrá extraer:

- `app.rs`: estado, mensajes, actualización y vista.
- `book.rs`: modelo de datos.
- `storage.rs`: lectura y escritura de JSON.

Estos módulos son decisiones de organización; iced no exige esos archivos.
