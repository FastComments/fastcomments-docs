Les sondages fonctionnent comme dans le widget web et s'activent depuis les mêmes paramètres de widget dans votre tableau de bord FastComments : `pollCreation` (qui peut joindre un sondage : désactivé, administrateurs et modérateurs, ou tout le monde) et `pollVoting` (utilisateurs connectés ou tout le monde).

- Les commentaires avec un sondage l'affichent sous leur texte. Les lecteurs votent en touchant une option, peuvent afficher ou masquer les résultats avant de voter, et peuvent changer leur vote. Les décomptes se mettent à jour en direct quand d'autres votent.
- Les sondages peuvent se clôturer à une date, masquer les résultats tant que vous n'avez pas voté, et rendre les votants visibles pour les administrateurs ou pour tout le monde (« Voir qui a voté »).
- Quand le spectateur peut créer des sondages, le champ de commentaire de `FastCommentsView` affiche un bouton de sondage qui ouvre le créateur de sondage. Un sondage est publié avec un commentaire, donc le texte du commentaire est obligatoire. Le champ du chat en direct n'affiche pas le bouton, comme le chat en direct web.

Le vote nécessite un utilisateur connecté, utilisez donc [SSO](#authentication-sso-readme-generated). Le serveur lie un vote anonyme à un cookie de session, que le client API ne conserve pas, donc avec `pollVoting` réglé sur tout le monde, un spectateur sans utilisateur SSO voit « Connectez-vous pour voter ». Les sondages qui masquent les résultats tant que vous n'avez pas voté ne reçoivent pas de mises à jour de décompte en direct ; ces votants voient les nouveaux chiffres à leur prochain chargement.

Pour publier un sondage depuis votre propre interface, passez un `CommentPollInput` à `postComment` :

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` et `sdk.getPollVoters(commentId:page:search:)` couvrent le vote et la liste des votants. Les couleurs des sondages sont listées sous [Poll Colors](#theming-readme-generated).