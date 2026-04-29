Se déclenche lorsqu'un commentaire atteint le seuil de votes nets configuré. Les votes nets sont `votesUp - votesDown`.

### Configuration requise

- **Vote threshold** - entier >= 1. Le déclencheur se produit sur le vote qui porte les votes nets exactement à ce nombre.

Si le seuil est 10 et qu'un commentaire passe de 9 à 10 votes nets, le déclencheur se déclenche une fois. Si un vote le fait ensuite passer de 10 à 11, le déclencheur ne se déclenche **pas** à nouveau - il ne se réactive pas à chaque vote supplémentaire au-delà du seuil.

### Contexte que reçoit l'agent

- Le commentaire, avec les comptes de votes actuels.
- La **direction du vote** (`up` ou `down`) du vote qui a provoqué le franchissement du seuil.
- Historique optionnel du fil / de l'utilisateur / contexte de la page tel que configuré.

### À noter

- Un commentaire qui monte à 10, retombe à 9, puis remonte à 10 déclenchera le déclencheur deux fois. Il n'y a pas d'état « fired once » par commentaire - si vous avez besoin de cette sémantique, faites en sorte que l'agent enregistre une [note mémoire](#tools-overview) lors de la première exécution et vérifiez-la lors des exécutions suivantes.
- Le seuil est toujours un nombre de votes **nets**, pas le nombre brut d'upvotes. Un commentaire avec 12 up et 2 down a un net de 10 et déclenche le déclencheur ; un autre avec 10 up et 0 down déclenche également.
- Les franchissements dus uniquement à un down-vote sont possibles - un commentaire passant de 11 à 10 à cause d'un down-vote déclenche aussi. Le paramètre `voteDirection` dans le contexte indique à l'agent de quelle direction est venu le franchissement du seuil.

### Utilisations courantes

- **Pinning** - le [Top Comment Pinner template](#template-top-comment-pinner) est construit autour de ce déclencheur.
- **Promotion / featured comment workflows** - émettre un événement via [Webhooks](#webhooks-overview) pour qu'un système externe puisse promouvoir le commentaire ailleurs sur votre site.
- **Engagement tracking** - enregistrer une note mémoire sur l'utilisateur qui a rédigé le commentaire afin que d'autres agents sachent qu'il a produit du contenu populaire.

### Réglage

Le seuil approprié dépend de la communauté. Surveillez l'[Run History](#run-history) pendant quelques jours avec un seuil bas (5) pour voir à quelle fréquence il se déclenche. Augmentez le seuil jusqu'à ce que la fréquence des déclenchements corresponde au rythme que vous souhaitez réellement.