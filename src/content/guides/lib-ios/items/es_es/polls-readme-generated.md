Las encuestas funcionan igual que en el widget web, y se activan desde la misma configuración del widget en tu panel de FastComments: `pollCreation` (quién puede adjuntar una encuesta: deshabilitado, administradores y moderadores, o todos) y `pollVoting` (usuarios conectados o cualquier persona).

- Los comentarios con una encuesta la muestran bajo su texto. Los lectores votan tocando una opción, pueden mostrar u ocultar los resultados antes de votar y pueden cambiar su voto. Los recuentos se actualizan en vivo a medida que otros votan.
- Las encuestas pueden cerrarse en una fecha, ocultar los resultados hasta que votes y hacer visibles a los votantes para los administradores o para todos ("Ver quién votó").
- Cuando el espectador puede crear encuestas, la entrada de comentarios de `FastCommentsView` muestra un botón de encuesta que abre el creador de encuestas. Una encuesta se publica con un comentario, por lo que el texto del comentario es obligatorio. La entrada del chat en vivo omite el botón, como el chat en vivo web.

Votar requiere un usuario conectado, así que usa [SSO](#authentication-sso-readme-generated). El servidor vincula un voto anónimo a una cookie de sesión, que el cliente de la API no conserva, por lo que con `pollVoting` configurado para cualquier persona, un espectador sin usuario de SSO ve "Inicia sesión para votar". Las encuestas que ocultan los resultados hasta que votas no reciben actualizaciones de recuento en vivo; esos votantes ven los nuevos números en su próxima carga.

Para publicar una encuesta desde tu propia interfaz, pasa un `CommentPollInput` a `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` y `sdk.getPollVoters(commentId:page:search:)` cubren la votación y la lista de votantes. Los colores de las encuestas se enumeran en [Colores de encuestas](#theming-readme-generated).