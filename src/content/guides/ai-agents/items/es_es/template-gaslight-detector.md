**Template ID:** `gaslight_detector`

El Detector de gaslighting vigila las ediciones de comentarios que reescriben la historia en medio de una conversación: ese tipo en el que un autor cambia el significado de un comentario anterior después de que ya se hayan escrito respuestas, dejando las respuestas posteriores fuera de contexto o incorrectas. Cuando el agente decide que una edición cruza esa línea, restaura el texto original y envía un DM al autor para explicarlo.

Esta es una plantilla de mayor riesgo porque modifica contenido de usuarios. Ejecútala en [modo dry-run](#dry-run-mode) más tiempo del que lo harías con una plantilla de solo lectura, y coloca `edit_comment` detrás de [approval](#approval-workflow) hasta que confíes en el juicio del modelo con respecto a tu tráfico.

### Desencadenantes

- **Comment edited** (`COMMENT_EDIT`) - el agente compara el texto nuevo y el anterior y decide si la edición deformó las respuestas que ya existen.

Consulta [Trigger: Comment Edited](#trigger-comment-edit) para ver la carga completa, incluyendo el texto del comentario anterior y el recuento de respuestas en el momento de la edición.

### Herramientas permitidas

- [`edit_comment`](#tool-edit-comment) - se usa para restaurar el texto original cuando la edición se considera gaslighting.
- [`warn_user`](#tool-warn-user) - emite una advertencia suave que el usuario verá en su próxima visita.
- [`send_dm`](#tools-overview) - el canal de explicación; el usuario recibe un mensaje directo describiendo por qué su edición fue revertida.

No puede banear, marcar como spam, votar ni publicar nuevos comentarios: el ámbito de acción es intencionalmente limitado.

### Recomendaciones antes de ponerlo en producción

- **Restringe `edit_comment` mediante [approval](#approval-workflow).** Revertir un comentario es visible para el autor y para cualquiera que haya visto la versión editada, por lo que un falso positivo resulta embarazoso. Mantén las aprobaciones activadas hasta que el modo dry-run muestre que el agente es consistente.
- **Ajusta el prompt con lo que cuenta como gaslighting en tu sitio.** El prompt por defecto es breve a propósito. Dale al modelo ejemplos concretos: "cambiar una afirmación de sí/no", "borrar un número que las respuestas citan", "añadir una frase hostil después de que se publicaron respuestas" —y no-ejemplos explícitos como correcciones de faltas de ortografía, limpieza de formato o añadir fuentes.
- **Usa el recuento de respuestas del contexto del desencadenante.** Las ediciones de comentarios con cero respuestas no pueden deformar una conversación; el prompt debe indicar al modelo que omita esos casos.
- **Marca la opción "Include commenter's trust factor, account age, ban history, and recent comments"** en [Context Options](#context-options). El modelo es mucho menos agresivo cuando puede ver una cuenta de larga data y de buena fe.
- **Considera una breve ventana de gracia para ediciones en el prompt.** Muchas ediciones dentro de los primeros 30-60 segundos son correcciones de errores tipográficos; indica al modelo que ignore ediciones tan rápidas.

### Ventana recomendada en modo dry-run

Ejecuta al menos dos semanas de tráfico real en [modo dry-run](#dry-run-mode) antes de cambiar a Activado, y revisa cada edición marcada durante ese periodo. Usa [Test Runs (Replays)](#test-runs-replays) para reproducir los últimos 30 días de ediciones contra el agente antes de ponerlo en producción.

---