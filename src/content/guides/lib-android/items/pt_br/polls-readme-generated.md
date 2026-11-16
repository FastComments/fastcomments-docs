Enquetes funcionam da mesma forma que no widget web e são ativadas nas mesmas configurações do widget no seu painel FastComments (ou com `pollCreation` / `pollVoting` acima).

- Comentários com uma enquete a exibem abaixo do texto. Os leitores votam tocando em uma opção, podem mostrar ou ocultar os resultados antes de votar e podem alterar seu voto. As contagens são atualizadas em tempo real conforme outros votam.
- As enquetes podem fechar em uma data, ocultar os resultados até que você vote e tornar os eleitores visíveis para administradores ou para todos ("Ver quem votou").
- Quando o visualizador pode criar enquetes, o campo de comentários do `FastCommentsView` mostra um botão de enquete que abre o construtor de enquetes. A linha da barra de ferramentas aparece para isso mesmo que você nunca tenha ativado a barra de ferramentas; `sdk.setCommentToolbarEnabled(false)` a oculta, incluindo o botão de enquete, e `setPollCreationEnabled(false)` no campo remove apenas o botão de enquete. Chame `setCommentToolbarEnabled` antes de criar o `FastCommentsView`, pois o campo lê essa configuração ao receber o SDK; para uma view já na tela, chame `getBottomCommentInput().setToolbarVisible(false)` em vez disso. O campo do chat ao vivo a omite, como no chat ao vivo da web.

Para publicar uma enquete a partir da sua própria UI, passe um `CommentPollInput` para `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Opcional: uma data de fechamento ISO 8601, um valor PollPrivacy e resultados ocultos até você votar.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` e `sdk.getPollVoters(commentId, page, search, callback)` cobrem a votação e a lista de eleitores. As cores das enquetes estão listadas em THEMING.md.