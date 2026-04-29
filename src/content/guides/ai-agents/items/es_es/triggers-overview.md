Un **disparador** es un evento que despierta a un agente. Cada agente puede tener uno o más disparadores definidos.

### La lista completa

| Disparador | Cuándo se activa |
|---|---|
| [Comentario agregado](#trigger-comment-add) | Se publica un nuevo comentario. |
| [Comentario editado](#trigger-comment-edit) | Se edita un comentario. El texto anterior se incluye en el contexto del agente. |
| [Comentario eliminado](#trigger-comment-delete) | Se elimina un comentario. |
| [Comentario fijado](#trigger-comment-pin) | Se fija un comentario (por cualquier persona, incluyendo un moderador u otro agente). |
| [Comentario desafijado](#trigger-comment-unpin) | Se desafija un comentario. |
| [Comentario bloqueado](#trigger-comment-lock) | Se bloquea un comentario (no se permiten más respuestas). |
| [Comentario desbloqueado](#trigger-comment-unlock) | Se desbloquea un comentario. |
| [Comentario supera el umbral de votos](#trigger-comment-vote-threshold) | Los votos netos de un comentario alcanzan el umbral configurado. |
| [Comentario supera el umbral de banderas](#trigger-comment-flag-threshold) | La cantidad de banderas de un comentario alcanza exactamente el umbral configurado. |
| [Usuario publica su primer comentario](#trigger-new-user-first-comment) | Un usuario publica su primer comentario en este sitio. |
| [Comentario marcado automáticamente como spam](#trigger-comment-auto-spammed) | Un comentario es marcado automáticamente como spam por el motor antispam. |
| [Moderador revisa comentario](#trigger-moderator-reviewed) | Un moderador marca un comentario como revisado. |
| [Moderador aprueba comentario](#trigger-moderator-approved) | Un moderador aprueba un comentario. |
| [Moderador marca como spam](#trigger-moderator-spammed) | Un moderador marca un comentario como spam. |
| [Moderador otorga insignia](#trigger-moderator-awarded-badge) | Un moderador otorga una insignia a un usuario. |

### Varios disparadores por agente

Un agente puede suscribirse a cualquier combinación de disparadores: la [Plantilla de Moderador](#template-moderator) se suscribe, por ejemplo, tanto a `COMMENT_ADD` como a `COMMENT_FLAG_THRESHOLD`. Cada evento activa el agente una vez con el contexto de ese evento.

### Qué impide que un agente se active

Un evento de disparador suscrito **no** activa al agente si se cumple cualquiera de las siguientes condiciones:

- El [estado](#status-states) del agente es **Deshabilitado**.
- El [alcance de URL o localización](#scope-url-locale) del agente no coincide con el comentario que desencadena.
- El [presupuesto diario, mensual o de tasa](#budgets-overview) del agente está agotado: el disparador se registra como **descartado** con una razón. Véase [Razones de descarte](#drop-reasons).
- La concurrencia para este agente está saturada (limitada por agente).
- El tenant del agente tiene facturación inválida.
- La acción que desencadenó fue realizada por un bot u otro agente (prevención de bucles).
- El disparador correspondía a un comentario que ya ha sido procesado por este agente dentro de la ventana de desduplicación.

Cuando un disparador suscrito se activa con éxito, el [Historial de ejecución](#run-history) del agente muestra una fila con el estado **Iniciado** que pasa a **Éxito** o **Error** cuando la ejecución finaliza.

### Umbrales de votos y banderas

Dos disparadores — **Comment Crosses Vote Threshold** y **Comment Crosses Flag Threshold** — requieren un umbral numérico en el formulario de edición. El disparador se activa en el momento en que el recuento cruza el valor configurado (específicamente, el disparador de umbral de banderas se activa cuando `flagCount === flagThreshold`, así que elegir 1 significa «activarse con la primera bandera», y elegir 5 significa «activarse cuando llegue la quinta bandera»).

### Disparadores diferidos

Cualquier disparador puede ser diferido para que el agente se ejecute más tarde, por ejemplo después de que los votos/banderas/respuestas hayan tenido tiempo de asentarse. Véase [Disparadores diferidos](#trigger-deferred-delay).

### Prevención de bucles

Para evitar bucles infinitos, los comentarios **escritos por un agente** llevan un `botId`. Los disparadores de nuevo comentario ignoran los comentarios con un `botId`.

El efecto neto: los agentes pueden actuar en respuesta a acciones *humanas* en su tenant, pero las acciones originadas por agentes nunca activan ningún disparador de agente. Esto se aplica a todos los tipos de disparador.

### REPLAY: el disparador interno

También existe un tipo de disparador interno `REPLAY` usado por la función [Ejecuciones de prueba (Reproducciones)](#test-runs-replays). No puede seleccionarlo en el formulario de edición: existe para que las ejecuciones de reproducción estén etiquetadas de forma distinta en el historial de ejecuciones y excluidas de las vistas de ejecuciones en vivo.

---