La herramienta Edit permite al agente reemplazar el texto de un comentario existente. Es destructiva en una forma en la que la mayoría de otras herramientas de moderación no lo son: sobrescribe contenido escrito por el usuario. Resérvala para casos estrechos y claros.

### What it does

El agente pasa un ID de comentario y un cuerpo de reemplazo. La plataforma escribe el nuevo texto en el comentario y registra una entrada `TextChanged` en el registro de auditoría del comentario que captura tanto el texto antiguo como el nuevo. El original nunca se pierde: los moderadores pueden leer lo que decía el comentario antes de que el agente lo editara.

El reemplazo pasa por la misma tubería de renderizado que una edición humana: el enmascaramiento de groserías, el análisis de menciones, la extracción de hashtags y el manejo de enlaces/imágenes se comportan exactamente como si el autor original hubiera enviado el nuevo texto.

### Scope

Como con toda herramienta que muta comentarios, Edit está restringida a la allowlist del trigger: el agente solo puede editar el comentario sobre el que se activó el trigger, su padre, u otro comentario dentro del alcance del mismo contexto del trigger. Un intento de inyección de prompt para "editar el comentario XYZ" donde XYZ no está relacionado será rechazado en el servidor antes de que el ejecutor se ejecute.

### Loops

Cuando el agente edita un comentario, la plataforma dispara un `COMMENT_EDIT` trigger como lo haría para una edición humana, pero **suprime el despacho a otros agentes**. Esto evita que dos agentes que escuchan `COMMENT_EDIT` se respondan mutuamente en efecto ping-pong por sus ediciones.

### When to allow it

Para agentes que manejan la redacción de PII, o para agentes resumidores/digest que se autoeditan. La mayoría de los agentes de moderación **no** necesitan esta herramienta: mark-spam, warn, and ban cubren el ciclo de vida típico.

### Approvals

**Considere seriamente protegerlo mediante aprobación**, especialmente mientras construye confianza en el agente. Que un agente reescriba las palabras de un usuario es una acción que la comunidad notará y ante la que reaccionará, y es más difícil de "deshacer" reputacionalmente que una eliminación.

### See also

- [Trigger: Comment Edited](#trigger-comment-edit) - el trigger que se activa cuando cambia el texto de un comentario.
- [Approval Workflow](#approval-workflow) - cómo proteger la herramienta mediante revisión humana.