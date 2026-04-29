La sección **Context** en el formulario de edición controla cuánta información recibe el agente en cada ejecución. Más contexto produce mejores decisiones pero aumenta el coste en tokens por ejecución, así que solo quieres lo que el agente realmente necesita.

### What's always included

Incluso con todas las casillas desmarcadas, el mensaje de contexto del agente incluye:

- El **tipo de evento desencadenante** (por ejemplo, `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- La **URL de la página y el ID de la URL** (cuando se conoce).
- El **comentario** que desencadenó la ejecución, si lo hay - ID, author user ID, author display name, comment text, vote counts, flag count, spam/approved/reviewed flags, parent ID. El correo electrónico del autor **nunca** se envía al proveedor de LLM (minimización de PII).
- El **texto del comentario previo** para los desencadenantes `COMMENT_EDIT` (para que el agente pueda comparar antes/después).
- La **dirección del voto** para los desencadenantes `COMMENT_VOTE_THRESHOLD`.
- El **ID del usuario que desencadenó** y el **ID de la insignia** (para desencadenantes de insignias de moderador).
- El **catálogo de insignias** de tu tenant (name, display label, description) cuando al agente se le permite otorgar insignias, para que pueda elegir una apropiada sin que tengas que listar las insignias en el prompt.

Todo texto no confiable - cuerpos de comentarios, nombres de autores, títulos de páginas, el propio documento de directrices - está **encerrado** en el mensaje de contexto con marcadores como `<<<COMMENT_TEXT>>> ... <<<END>>>`. El prompt del sistema de la plataforma instruye al modelo a no seguir jamás instrucciones dentro de esos cercos. Esta es la defensa contra inyección de prompt de la plataforma; no necesitas repetirla en tu prompt.

### The three checkboxes

#### Include parent comment and prior replies in the same thread

Agrega:
- El **comentario padre** - ID, author, text.
- **Respuestas hermanas** - las respuestas anteriores al mismo padre en el mismo hilo.

Útil para: cualquier agente que responda a un comentario en contexto (saludadores de bienvenida, resumidores de hilos, moderadores que leen respuestas en conversaciones).

Coste: pequeño a medio. Limitado por cuántos hermanos existan en un hilo dado.

#### Include commenter's trust factor, account age, ban history, and recent comments

Agrega el bloque **AUTHOR_HISTORY**:

- **Antigüedad de la cuenta en días** desde el registro.
- **Trust factor (0-100)** - la puntuación de FastComments que resume cuánto se confía en el usuario en este sitio. Véase la página [Detección de spam](/guide-moderation.html#spam-detection) en la guía de moderación.
- **Recuento previo de baneos.**
- **Total de comentarios en este sitio.**
- **Recuento de contenido duplicado** - si el usuario ha publicado texto idéntico recientemente (señal anti-spam).
- **Señal de cuentas cruzadas por la misma IP** - recuento de comentarios desde la misma IP bajo otras cuentas (señal de cuentas alternativas). El hash de la IP nunca se envía al LLM.
- **Comentarios recientes** - hasta 5 de los comentarios más recientes del usuario, cada uno truncado a 300 caracteres, encuadrados como texto no confiable.

Útil para: cualquier agente de moderación. Sin esto, el modelo banea cuentas nuevas y usuarios de buena fe de largo tiempo con la misma postura.

Coste: medio. Los comentarios recientes añaden la mayor cantidad de tokens.

#### Include page title, subtitle, description, and meta tags

Agrega el bloque **PAGE_CONTEXT** - title, subtitle, description, y cualquier meta tag que FastComments haya capturado para la página.

Útil para: saludadores de bienvenida y resumidores de hilos, donde saber de qué trata la página mejora sustancialmente la calidad de la salida.

Coste: pequeño.

### Community guidelines

El cuarto campo, **Community guidelines**, es un bloque de políticas de texto libre incluido en el mensaje de contexto con rol de usuario en cada ejecución, encerrado como texto no confiable de la misma manera que los cuerpos de comentarios y otros contenidos proporcionados por el usuario. El agente lo lee como texto de política pero la plataforma no lo trata como una instrucción del sistema. Véase [Community Guidelines](#community-guidelines) para qué poner en él.

### Adding context selectively

Estas casillas se aplican por agente, no globalmente. Un patrón común:

- Saludador de bienvenida: page context **on**, thread context **off**, user history **off**.
- Moderador: thread context **off**, user history **on**, page context **off**.
- Resumidor de hilos: thread context **on**, page context **on**, user history **off**.

Busca el contexto mínimo que un agente necesita para ser correcto en las llamadas que realmente hace: el contexto extra cuesta tokens en cada ejecución, incluso cuando el agente no lo usa.