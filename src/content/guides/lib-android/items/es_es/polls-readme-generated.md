Las encuestas funcionan igual que en el widget web, y se activan desde la misma configuración del widget en tu panel de FastComments (o con `pollCreation` / `pollVoting` mencionados arriba).

- Los comentarios con una encuesta la muestran debajo de su texto. Los lectores votan tocando una opción, pueden mostrar u ocultar los resultados antes de votar y pueden cambiar su voto. Los recuentos se actualizan en vivo a medida que otros votan.
- Las encuestas pueden cerrarse en una fecha, ocultar los resultados hasta que votes y hacer visibles los votantes para los administradores o para todos ("Ver quién votó").
- Cuando el usuario puede crear encuestas, la entrada de comentarios de `FastCommentsView` muestra un botón de encuesta que abre el creador de encuestas. La fila de la barra de herramientas aparece para ello incluso si nunca activaste la barra de herramientas; `sdk.setCommentToolbarEnabled(false)` la oculta, incluido el botón de encuesta, y `setPollCreationEnabled(false)` en la entrada elimina solo el botón de encuesta. Llama a `setCommentToolbarEnabled` antes de crear el `FastCommentsView`, ya que la entrada lo lee cuando recibe el SDK; para una vista que ya está en pantalla, llama en su lugar a `getBottomCommentInput().setToolbarVisible(false)`. La entrada del chat en vivo lo omite, igual que el chat en vivo web.

Para publicar una encuesta desde tu propia interfaz, pasa un `CommentPollInput` a `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Opcional: una fecha de cierre ISO 8601, un valor de PollPrivacy y resultados ocultos hasta que votes.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` y `sdk.getPollVoters(commentId, page, search, callback)` cubren la votación y la lista de votantes. Los colores de las encuestas están listados en THEMING.md.