---
Se dispara cuando se elimina un comentario.

### Contexto que recibe el agente

- El comentario que acaba de eliminarse - texto, autor, página.
- Contexto opcional de hilo / historial de usuario / página según lo configurado.

### A destacar

- Se activa tanto para **eliminaciones suaves** (cuando el comentario está oculto pero se conserva para auditoría) como para **eliminaciones definitivas** (cuando el comentario se elimina por completo). El manejador del disparador resuelve el comentario desde la canalización de eliminación en cascada; lo que el agente ve es el último estado conocido.
- Una vez que un comentario se elimina por completo, las herramientas que lo apuntan (`pin_comment`, `mark_comment_spam`, etc.) para ese ID de comentario fallarán.

### Usos comunes

- **Reenvío de auditoría vía [Webhooks](#webhooks-overview)** - emitir un evento `trigger.succeeded` para que un sistema externo registre lo que se eliminó.
- **Escrituras en memoria** - hacer que el agente registre una [nota de memoria](#tools-overview) sobre un patrón de eliminación (el comentario eliminado fue el tercero del usuario en 24 horas, etc.).
- **Efectos entre hilos** - detectar cuando una eliminación cambia la estructura de un hilo que el agente había resumido previamente, y considerar si volver a resumirlo.

### Nota sobre el coste de operación

Si tienes un sitio con un alto volumen de eliminaciones (moderación humana intensiva), este disparador puede activarse con frecuencia.

---