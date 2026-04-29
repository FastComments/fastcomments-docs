La sección **Contexto** en el formulario de edición controla cuánta información recibe el agente en cada ejecución. Más contexto produce mejores decisiones pero aumenta el coste en tokens por ejecución, por lo que solo quieres lo que el agente realmente necesita.

### Qué se incluye siempre

Incluso con todas las casillas desmarcadas, el mensaje de contexto del agente incluye:

- El **tipo de evento desencadenante** (p. ej. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- La **URL de la página y el ID de URL** (cuando se conozcan).
- El **comentario** que desencadenó la ejecución, si lo hay: ID, ID de usuario del autor, nombre para mostrar del autor, texto del comentario, recuentos de votos, recuento de banderas, indicadores de spam/aprobado/revisado, ID del padre. El correo electrónico del autor **nunca** se envía al proveedor de LLM (minimización de PII).
- El **texto del comentario previo** para los desencadenantes `COMMENT_EDIT` (para que el agente pueda comparar antes/después).
- La **dirección del voto** para los desencadenantes `COMMENT_VOTE_THRESHOLD`.
- El **ID del usuario que desencadenó** el evento y el **ID de la insignia** (para desencadenantes de insignias de moderador).

Todo texto no confiable - cuerpos de comentarios, nombres de autores, títulos de página, el propio documento de directrices - está **encerrado** en el mensaje de contexto con marcadores como `<<<COMMENT_TEXT>>> ... <<<END>>>`. El prompt del sistema de la plataforma instruye al modelo a no seguir instrucciones dentro de esas cercas. Esta es la defensa de la plataforma contra la inyección de prompts; no necesitas repetirla en tu prompt.

### Las tres casillas

#### Incluir el comentario padre y las respuestas previas en el mismo hilo

Agrega:
- El **comentario padre** - ID, autor, texto.
- **Respuestas hermanas** - las respuestas previas al mismo padre en el mismo hilo.

Útil para: cualquier agente que responda a un comentario en contexto (saludadores de bienvenida, resumidores de hilos, moderadores que leen respuestas en conversaciones).

Costo: pequeño a medio. Limitado por cuántas respuestas hermanas existan en un hilo dado.

#### Incluir el factor de confianza del comentarista, antigüedad de la cuenta, historial de baneos y comentarios recientes

Agrega el bloque **AUTHOR_HISTORY**:

- **Antigüedad de la cuenta en días** desde el registro.
- **Factor de confianza (0-100)** - la puntuación de FastComments que resume cuán confiable es el usuario en este sitio. Consulta la página de [Detección de spam](/guide-moderation.html#spam-detection) en la guía de moderación.
- **Número de baneos previos.**
- **Total de comentarios en este sitio.**
- **Recuento de contenido duplicado** - si el usuario ha publicado texto idéntico recientemente (señal anti-spam).
- **Señal de cuentas cruzadas por la misma IP** - recuento de comentarios desde la misma IP bajo otras cuentas (señal de cuentas alternativas). El hash de la IP nunca se envía al LLM.
- **Comentarios recientes** - hasta 5 de los comentarios más recientes del usuario, cada uno truncado a 300 caracteres, encerrados como texto no confiable.

Útil para: cualquier agente de moderación. Sin esto, el modelo banea cuentas nuevas y usuarios de buena fe de larga trayectoria con la misma postura.

Costo: medio. Los comentarios recientes añaden la mayor cantidad de tokens.

#### Incluir título de la página, subtítulo, descripción y metaetiquetas

Agrega el bloque **PAGE_CONTEXT**: título, subtítulo, descripción y cualquier metaetiqueta que FastComments haya capturado para la página.

Útil para: saludadores de bienvenida y resumidores de hilos, donde saber de qué trata la página mejora sustancialmente la calidad de la salida.

Costo: pequeño.

### Directrices de la comunidad

El cuarto campo, **Community guidelines**, es un bloque de política de texto libre incluido en el mensaje de contexto con rol de usuario en cada ejecución, encerrado como texto no confiable de la misma manera que los cuerpos de comentarios y otros contenidos proporcionados por el usuario. El agente lo lee como texto de política pero la plataforma no lo trata como una instrucción del sistema. Consulta [Directrices de la comunidad](#community-guidelines) para saber qué poner en él.

### Añadir contexto selectivamente

Estas casillas se aplican por agente, no de forma global. Un patrón común:

- Saludador de bienvenida: contexto de página **activado**, contexto de hilo **desactivado**, historial de usuario **desactivado**.
- Moderador: contexto de hilo **desactivado**, historial de usuario **activado**, contexto de página **desactivado**.
- Resumidor de hilo: contexto de hilo **activado**, contexto de página **activado**, historial de usuario **desactivado**.

Busca el mínimo contexto que un agente necesite para ser correcto en las llamadas que realmente hace: el contexto extra cuesta tokens en cada ejecución, incluso cuando el agente no lo utiliza.