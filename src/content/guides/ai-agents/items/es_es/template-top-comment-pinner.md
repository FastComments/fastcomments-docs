**Template ID:** `top_comment_pinner`

El Fijador de Comentarios Principales vigila los comentarios de primer nivel que superan un umbral de votos y los fija, reemplazando lo que estuviera fijado anteriormente en el mismo hilo.

El prompt incorporado instruye al agente a omitir respuestas (fijar funciona en hilos, por lo que fijar una respuesta rara vez es útil) y a filtrar contenido promocional (para que el agente no potencie el spam de enlaces populares).

### Desencadenadores

- **A comment crosses a vote threshold** (`COMMENT_VOTE_THRESHOLD`, default vote threshold: 10).

El disparador se activa cuando los votos netos del comentario (`up - down`) alcanzan el umbral configurado. Ajusta el número en el formulario de edición según la actividad de tus hilos - 10 es un valor sensato por defecto para sitios moderadamente activos.

### Herramientas permitidas

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Fijar no es destructivo - puede revertirse al instante - por lo que esta plantilla normalmente se ejecuta sin aprobaciones.

### Recomendaciones antes de publicar

- **Marca "Incluir el comentario padre y respuestas anteriores en el mismo hilo"** en [Opciones de contexto](#context-options). Sin contexto de hilo, el agente no puede determinar con fiabilidad si ya existe un comentario fijado para desfijar.
- **Ajusta el umbral de votos** a tu sitio. En hilos concurridos 10 ocurre con demasiada frecuencia; en hilos tranquilos puede que 10 nunca ocurra.
- **Considera limitar por URL** si solo quieres comentarios fijados en ciertas secciones de tu sitio - por ejemplo, hilos de noticias, pero no los hilos de anuncios.

### Nota sobre fijado duplicado

El prompt del agente le indica que primero desfije antes de fijar, pero si el modelo omite ese paso la plataforma en sí no aplica una regla de un fijado por hilo (puedes tener varios). Si el fijado duplicado es un problema en tu sitio, coloca `pin_comment` detrás de una aprobación y revisa cada uno - o escribe un prompt más estricto.