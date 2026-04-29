**Template ID:** `top_comment_pinner`

El Top Comment Pinner vigila los comentarios de primer nivel que cruzan un umbral de votos y los fija, reemplazando lo que estuviera fijado previamente en el mismo hilo.

### Mensaje inicial incorporado

[inline-code-attrs-start title = 'Mensaje inicial de la plantilla Top Comment Pinner'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

La instrucción "do not pin replies" importa: fijar funciona a nivel de hilos, por lo que fijar una respuesta rara vez es útil. El filtro "non-promotional" evita que el agente potencie un comentario popular de spam con enlaces.

### Desencadenadores

- **Un comentario supera un umbral de votos** (`COMMENT_VOTE_THRESHOLD`, umbral de votos por defecto: 10).

El desencadenador se activa cuando los votos netos del comentario (`up - down`) alcanzan el umbral configurado. Ajusta el número en el formulario de edición según la actividad de tus hilos: 10 es un valor sensato para sitios moderadamente activos.

### Herramientas permitidas

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fijar no es destructivo: se puede revertir al instante, por lo que esta plantilla normalmente se ejecuta sin aprobaciones.

### Recomendaciones antes de poner en producción

- **Marca "Incluir el comentario padre y respuestas previas en el mismo hilo"** en [Opciones de contexto](#context-options). Sin el contexto del hilo el agente no puede determinar con fiabilidad si ya existe un comentario fijado que deba desanclarse.
- **Ajusta el umbral de votos** según tu sitio. En hilos muy activos 10 ocurre con demasiada frecuencia; en hilos tranquilos 10 puede que nunca ocurra.
- **Considera limitar por URL** si solo quieres comentarios fijados en ciertas secciones de tu sitio — por ejemplo hilos de noticias, pero no hilos de anuncios.

### Nota sobre pines duplicados

El prompt del agente le indica que primero desancle antes de fijar, pero si el modelo omite ese paso la propia plataforma no aplica una regla de "un pin por hilo" (puedes tener varios). Si los pines duplicados son un problema en tu sitio, condiciona `pin_comment` a aprobación y revisa cada uno, o escribe un prompt más estricto.

---