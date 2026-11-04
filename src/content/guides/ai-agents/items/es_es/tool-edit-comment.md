La herramienta Edit permite al agente reemplazar el texto de un comentario existente. Es destructiva de una forma en la que la mayoría de las otras herramientas de moderación no lo son: sobrescribe contenido creado por el usuario. Resérvela para casos reducidos y claros.

### Qué hace

El agente pasa un ID de comentario y un cuerpo de reemplazo. La plataforma escribe el nuevo texto en el comentario y registra una entrada `TextChanged` en el registro de auditoría del comentario, capturando tanto el texto antiguo como el nuevo. El original nunca se pierde: los moderadores pueden leer lo que decía el comentario antes de que el agente lo editara.

El reemplazo pasa por el mismo pipeline de renderizado que una edición humana: el enmascaramiento de palabras ofensivas, el análisis de menciones, la extracción de hashtags y el manejo de enlaces/imágenes se comportan exactamente como si el autor original hubiera enviado el nuevo texto.

### Alcance

Como todas las herramientas que modifican comentarios, Edit está limitada a la lista de permitidos del trigger: el agente solo puede editar el comentario en el que se disparó el trigger, su comentario padre u otro comentario dentro del alcance del mismo contexto de trigger. Un intento de inyección de prompt para "editar el comentario XYZ" donde XYZ no está relacionado será rechazado en el servidor antes de que se ejecute el executor.

### Bucles

Cuando el agente edita un comentario, la plataforma dispara un trigger `COMMENT_EDIT` como lo haría para una edición humana, pero **suprime el envío a otros agentes**. Esto evita que dos agentes que escuchan `COMMENT_EDIT` se pasen el balón con las ediciones del otro.

### Cuándo permitirlo

Para agentes que manejan la eliminación de datos PII, o para agentes de resumen/digest que se autoeditan. La mayoría de los agentes de moderación **no** necesitan esta herramienta: mark-spam, warn y ban cubren el ciclo de vida típico.

### Aprobaciones

**Considere seriamente requerir aprobación**, especialmente mientras construye confianza en el agente. Que un agente reescriba las palabras de un usuario es el tipo de acción que una comunidad notará y a la que reaccionará, y es más difícil de "deshacer" en términos de reputación que una eliminación.

### Véase también

- [Trigger: Comment Edited](#trigger-comment-edit) - el trigger que se dispara cuando cambia el texto de un comentario.
- [Approval Workflow](#approval-workflow) - cómo requerir revisión humana para la herramienta.