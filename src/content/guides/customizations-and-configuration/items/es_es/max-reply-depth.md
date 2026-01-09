[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Por defecto, FastComments permite un anidamiento ilimitado de respuestas, creando una estructura de hilos donde los usuarios pueden responder a respuestas indefinidamente.

La opción maxReplyDepth te permite limitar hasta qué profundidad pueden llegar los hilos de respuestas. Cuando se alcanza la profundidad máxima, los usuarios ya no verán un botón de responder en los comentarios en ese nivel.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Con maxReplyDepth establecido en 2:
- Los usuarios pueden comentar en el nivel superior (profundidad 0)
- Los usuarios pueden responder a los comentarios de nivel superior (profundidad 1)
- Los usuarios pueden responder a esas respuestas (profundidad 2)
- No se permiten más respuestas más allá de la profundidad 2

Configurar a 1 solo permitiría respuestas a comentarios de nivel superior, creando una estructura de discusión más plana.

Establecer maxReplyDepth a 0 deshabilitaría todas las respuestas, permitiendo solo comentarios de nivel superior. Si no se especifica, las respuestas pueden anidarse sin límite.