Les sondages fonctionnent comme dans le widget web et s'activent depuis les mêmes paramètres du widget dans votre tableau de bord FastComments (ou avec `pollCreation` / `pollVoting` ci-dessus).

- Les commentaires avec un sondage l'affichent sous leur texte. Les lecteurs votent en touchant une option, peuvent afficher ou masquer les résultats avant de voter, et peuvent modifier leur vote. Les décomptes sont mis à jour en direct lorsque d'autres votent.
- Les sondages peuvent se clôturer à une date, masquer les résultats tant que vous n'avez pas voté, et rendre les votants visibles pour les administrateurs ou pour tout le monde (« See who voted »).
- Lorsque le lecteur peut créer des sondages, le champ de saisie des commentaires de `FastCommentsView` affiche un bouton de sondage qui ouvre le créateur de sondage. La ligne de barre d'outils apparaît pour celui-ci même si vous n'avez jamais activé la barre d'outils ; `sdk.setCommentToolbarEnabled(false)` la masque, bouton de sondage inclus, et `setPollCreationEnabled(false)` sur le champ de saisie ne supprime que le bouton de sondage. Appelez `setCommentToolbarEnabled` avant de créer le `FastCommentsView`, car le champ de saisie le lit lorsqu'il reçoit le SDK ; pour une vue déjà à l'écran, appelez plutôt `getBottomCommentInput().setToolbarVisible(false)`. Le champ de saisie du chat en direct l'omet, comme le chat en direct web.

Pour publier un sondage depuis votre propre interface, passez un `CommentPollInput` à `postComment` :

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Optional: an ISO 8601 close date, PollPrivacy value, and results hidden until you vote.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)` et `sdk.getPollVoters(commentId, page, search, callback)` couvrent le vote et la liste des votants. Les couleurs des sondages sont listées dans THEMING.md.