La sección **Context** del formulario de edición controla cuánta información recibe el agente en cada ejecución. Más contexto produce mejores decisiones pero aumenta el coste de tokens por ejecución, por lo que solo quieres lo que el agente realmente necesita.

### Qué se incluye siempre

Incluso con todas las casillas desmarcadas, el mensaje de contexto del agente incluye:

- El **tipo de evento desencadenante** (por ejemplo, `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- La **URL de la página y el URL ID** (cuando se conocen).
- El **comentario** que desencadenó la ejecución, si existe - ID, ID de usuario del autor, nombre visible del autor, texto del comentario, recuentos de votos, recuento de marcas, marcas de spam/aprobado/revisado, ID del padre. El correo electrónico del autor **nunca** se envía al proveedor de LLM (minimización de PII).
- El **texto del comentario anterior** para los desencadenantes `COMMENT_EDIT` (para que el agente pueda comparar antes/después).
- La **dirección del voto** para los desencadenantes `COMMENT_VOTE_THRESHOLD`.
- El **ID del usuario desencadenante** y el **badge ID** (para desencadenantes de badges de moderador).
- El **catálogo de badges** de tu tenant (nombre, etiqueta visible, descripción) cuando se permite al agente otorgar badges, para que el agente pueda elegir uno apropiado sin que tengas que detallar los badges en el prompt.

Todo el texto no confiable - cuerpos de comentarios, nombres de autores, títulos de páginas, el propio documento de directrices - está **delimitado** en el mensaje de contexto con marcadores como `<<<COMMENT_TEXT>>> ... <<<END>>>`. El system prompt de la plataforma instruye al modelo para que nunca siga instrucciones dentro de esos delimitadores. Esta es la defensa contra la inyección de prompts de la plataforma; no necesitas repetirla en tu prompt.

### Las tres casillas

#### Incluir el comentario padre y las respuestas previas del mismo hilo

Añade:
- El **comentario padre** - ID, autor, texto.
- **Respuestas hermanas** - las respuestas previas al mismo padre en el mismo hilo.

Útil para: cualquier agente que responda a un comentario en contexto (saludos de bienvenida, resumidores de hilos, moderadores que leen respuestas en conversaciones).

Coste: pequeño a medio. Limitado por cuántas respuestas hermanas existan en un hilo determinado.

#### Incluir el factor de confianza del comentarista, la antigüedad de la cuenta, el historial de baneos y los comentarios recientes

Añade el bloque **AUTHOR_HISTORY**:

- **Antigüedad de la cuenta en días** desde el registro.
- **Factor de confianza (0-100)** - la puntuación de FastComments que resume cuán confiable es el usuario en este sitio. Consulta la página [Spam Detection](/guide-moderation.html#spam-detection) en la guía de moderación.
- **Recuento de baneos previos.**
- **Total de comentarios en este sitio.**
- **Recuento de contenido duplicado** - si el usuario ha publicado texto idéntico recientemente (señal anti-spam).
- **Señal de cuentas cruzadas por misma IP** - recuento de comentarios desde la misma IP bajo otras cuentas (señal de cuentas alternativas). El hash de la IP en sí nunca se envía al LLM.
- **Comentarios recientes** - hasta 5 de los comentarios más recientes del usuario, cada uno truncado a 300 caracteres, delimitados como texto no confiable.

Útil para: cualquier agente de moderación. Sin esto, el modelo banea cuentas nuevas y usuarios antiguos de buena fe con la misma postura.

Coste: medio. Los comentarios recientes añaden la mayor cantidad de tokens.

#### Incluir el título de la página, subtítulo, descripción y meta tags

Añade el bloque **PAGE_CONTEXT** - título, subtítulo, descripción y cualquier meta tag que FastComments haya capturado para la página.

Útil para: saludos de bienvenida y resumidores de hilos, donde saber de qué trata la página mejora sustancialmente la calidad de la salida.

Coste: pequeño.

### Directrices de la comunidad

El cuarto campo, **Community guidelines**, es un bloque de política de texto libre incluido en el mensaje de contexto de rol de usuario en cada ejecución, delimitado como texto no confiable de la misma manera que se delimitan los cuerpos de comentarios y otros contenidos proporcionados por el usuario. El agente lo lee como texto de política, pero la plataforma no lo trata como una instrucción de sistema. Consulta [Community Guidelines](#community-guidelines) para saber qué incluir en él.

### Añadir contexto de forma selectiva

Estas casillas se aplican por agente, no globalmente. Un patrón común:

- Saludo de bienvenida: contexto de página **activado**, contexto de hilo **desactivado**, historial de usuario **desactivado**.
- Moderador: contexto de hilo **desactivado**, historial de usuario **activado**, contexto de página **desactivado**.
- Resumidor de hilos: contexto de hilo **activado**, contexto de página **activado**, historial de usuario **desactivado**.

Recurre al contexto mínimo que un agente necesita para ser correcto en las llamadas que realmente realiza - el contexto extra cuesta tokens en cada ejecución, incluso cuando el agente no lo usa.