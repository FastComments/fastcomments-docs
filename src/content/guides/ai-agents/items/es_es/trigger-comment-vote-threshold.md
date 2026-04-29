---
Se activa cuando el recuento neto de votos de un comentario alcanza el umbral configurado. Los votos netos son `votesUp - votesDown`.

### Configuración requerida

- **Umbral de votos** - entero >= 1. El disparador se activa con el voto que lleva los votos netos exactamente a ese número.

Si el umbral es 10 y un comentario pasa de 9 a 10 votos netos, el disparador se activa una vez. Si un voto lo sube de 10 a 11, el disparador **no** se activa de nuevo - no se vuelve a activar por cada voto adicional que supere el umbral.

### Contexto que recibe el agente

- El comentario, con los recuentos de votos actuales.
- La **dirección del voto** (`up` o `down`) del voto que provocó el cruce del umbral.
- Historial opcional del hilo / usuario / contexto de la página según esté configurado.

### Observaciones

- Un comentario que sube a 10, baja a 9 y vuelve a subir a 10 activará el disparador dos veces. No existe un estado por comentario de "fired once" - si necesita esa semántica, haga que el agente registre una [nota de memoria](#tools-overview) en la primera ejecución y la compruebe en ejecuciones posteriores.
- El umbral siempre es un recuento de votos **neto**, no de solo votos positivos. Un comentario con 12 up y 2 down tiene net 10 y activa el disparador; uno con 10 up y 0 down también lo activa.
- También son posibles cruces debidos únicamente a votos negativos: un comentario que pase de 11 a 10 por un down-vote también activa el disparador. El parámetro `voteDirection` en el contexto indica al agente desde qué dirección vino el cruce del umbral.

### Usos comunes

- **Fijado** - la [plantilla Top Comment Pinner](#template-top-comment-pinner) está construida alrededor de este disparador.
- **Promoción / flujos de trabajo de comentario destacado** - emitir un evento vía [Webhooks](#webhooks-overview) para que un sistema externo pueda promocionar el comentario en otra parte de su sitio.
- **Seguimiento de la participación** - registrar una nota de memoria sobre el usuario que escribió el comentario para que otros agentes sepan que ha producido contenido popular.

### Ajuste

El umbral adecuado depende de la comunidad. Observe [Run History](#run-history) durante unos días con un umbral bajo (5) para ver con qué frecuencia se activa. Aumente el umbral hasta que la frecuencia de activación coincida con la cadencia que realmente desea.

---