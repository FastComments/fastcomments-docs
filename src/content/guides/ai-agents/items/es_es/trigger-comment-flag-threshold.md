Se activa cuando el recuento de banderas de un comentario alcanza **exactamente** el umbral configurado.

### Configuración requerida

- **Umbral de banderas** - entero >= 1. El disparador se activa en el momento en que `flagCount === flagThreshold`. No se vuelve a activar en banderas posteriores al umbral.

Si el umbral es 3 y tres usuarios marcan el comentario, el agente se activa una vez en la tercera bandera. Una cuarta, quinta o sexta bandera **no** lo volverá a activar.

### Contexto que recibe el agente

- El comentario marcado.
- Contexto opcional de hilo / historial del usuario / página según esté configurado.
- El recuento de banderas está en el bloque del comentario como `Flag Count: N`.

### Notas importantes

- El disparador solo se activa cuando el comentario cruza el umbral desde abajo a través de la ruta de gestión de banderas de la plataforma (donde `didIncrement === true`). Escrituras directas en la BD que establezcan `flagCount` al valor del umbral no lo activan; las banderas por encima del umbral tampoco lo reactivan.
- No incluye quién marcó el comentario: las banderas son anónimas para el agente. Si quieres ver qué usuarios marcaron, recupéralos de tus propios datos.
- Se recomienda *encarecidamente* un retardo del disparador (ver [Disparadores diferidos](#trigger-deferred-delay)) en este disparador: las banderas suelen llegar en ráfagas durante un hilo acalorado, y un pequeño retraso permite que la situación se estabilice antes de que actúe el agente.

### Usos comunes

- **Revisión de moderación** - un comentario marcado es la señal canónica de "los humanos creen que esto podría ser malo". La [Plantilla de moderador](#template-moderator) se suscribe a este disparador por defecto con un umbral de banderas de 3.
- **Aumento de la cola de premoderación** - el agente realiza una pasada inicial y o bien marca el comentario para moderación (con `mark_comment_reviewed`) o lo escala más.
- **Contra brigadas** - combina este disparador con [contexto de historial de usuario](#context-options) y permite que el agente vea baneos previos/señales de contenido duplicado antes de actuar.

### Recomendaciones de combinación

Suscríbete a **ambos** `COMMENT_ADD` y `COMMENT_FLAG_THRESHOLD` si quieres un agente de moderación que detecte los casos obvios a primera vista y reevalúe los casos límite una vez que las banderas se acumulen. Los dos eventos se activan de forma independiente: el agente se ejecutará dos veces si ambos están suscritos y ambos se activan, pero la segunda ejecución verá el estado ya marcado.