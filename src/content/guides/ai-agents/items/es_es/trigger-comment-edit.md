---
Activa el agente cuando se edita un comentario.

### Contexto que recibe el agente

- El comentario en su forma actual (posterior a la edición).
- El **texto anterior del comentario** como un bloque delimitado separado (`PREVIOUS_TEXT`). Esto es exclusivo del disparador de edición: el agente puede comparar antes/después.
- Historial opcional de hilo/usuario/contexto de página según la configuración.

### Observaciones

- El disparador se activa para cualquier edición exitosa, incluidas las ediciones realizadas por moderadores en nombre de un usuario.
- A los agentes no se les expone ninguna herramienta para editar comentarios; los agentes no pueden editar comentarios.
- El texto anterior del comentario está cercado como entrada no confiable. El prompt del sistema de la plataforma recuerda al modelo que no debe seguir instrucciones que estén dentro de bloques cercados: esto importa aquí, porque un usuario malintencionado podría editar un comentario para insertar una carga útil de "ignora tus instrucciones previas" dirigida a cualquier agente que observe eventos de edición.

### Usos comunes

- **Detectar contenido camuflado** - un usuario edita un comentario previamente limpio para insertar spam después de que el moderador haya pasado a otra cosa.
- **Rastrear ediciones menores** - si tu comunidad trata las ediciones como eventos separados por cualquier motivo de auditoría.

### Nota sobre costos

Los disparadores de edición ven dos copias del texto del comentario (la versión nueva en el bloque estándar COMMENT, la versión anterior en el bloque PREVIOUS_TEXT). Para comentarios largos esto aproximadamente duplica el costo en tokens de la ejecución en comparación con un disparador `COMMENT_ADD` - tenlo en cuenta al presupuestar.

---