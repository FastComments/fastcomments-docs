Se activa cuando se ancla un comentario.

### Contexto que recibe el agente

- El comentario anclado.
- Historial del hilo / historial del usuario / contexto de la página opcionales según la configuración.

### Quién lo activa

- Un moderador que hace clic en la acción de anclar en la página de moderación o en el widget de comentarios.
- Un agente que llama a [`pin_comment`](#tools-overview).

Prevención de bucles: los eventos de anclado originados por un agente no activan ningún disparador de agente. Un anclado realizado por un agente interrumpe todo el despacho de agentes para ese evento, no solo el agente originador.

### Nota sobre el par

Los eventos de anclado y desanclado son disparadores separados. Suscríbase a ambos si desea un comportamiento simétrico. Vea [Disparador: Comentario Desanclado](#trigger-comment-unpin).

---