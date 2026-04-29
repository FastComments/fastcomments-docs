Se déclenche lorsqu'un commentaire atteint le seuil configuré pour le nombre de votes nets. Les votes nets sont `votesUp - votesDown`.

### Configuration requise

- **Seuil de votes** - entier >= 1. Le déclencheur se produit sur le vote qui porte le total net de votes à exactement ce nombre.

Si le seuil est 10 et qu'un commentaire passe de 9 à 10 votes nets, le déclencheur se déclenche une fois. Si un autre vote l'emmène ensuite de 10 à 11, le déclencheur ne se déclenche pas à nouveau - il ne se ré-exécute pas pour chaque vote supplémentaire au-delà du seuil.

### Contexte que reçoit l'agent

- Le commentaire, avec les totaux de votes actuels.
- La **direction du vote** (`up` ou `down`) du vote qui a provoqué le franchissement du seuil.
- Historique optionnel de fil / d'utilisateur / contexte de page tel que configuré.

### Remarques

- Un commentaire qui atteint 10, retombe à 9, puis remonte à 10 déclenchera le déclencheur deux fois. Il n'existe pas d'état "déclenché une fois" par commentaire - si vous avez besoin de cette sémantique, faites en sorte que l'agent enregistre une [memory note](#tools-overview) lors de la première exécution et vérifie sa présence lors des exécutions suivantes.
- Le seuil est toujours un nombre de votes **nets**, pas le nombre brut de votes positifs. Un commentaire avec 12 votes positifs et 2 votes négatifs a un net de 10 et déclenche le déclencheur ; un commentaire avec 10 votes positifs et 0 vote négatif déclenche également.
- Des franchissements uniquement dus à des votes négatifs sont possibles - un commentaire passant de 11 à 10 à cause d'un vote négatif déclenche aussi. Le paramètre `voteDirection` dans le contexte indique à l'agent de quel sens provenait le franchissement du seuil.

### Utilisations courantes

- **Épinglage** - le [Top Comment Pinner template](#template-top-comment-pinner) est construit autour de ce déclencheur.
- **Promotion / flux de travail pour commentaires mis en avant** - émettez un événement via [Webhooks](#webhooks-overview) afin qu'un système externe puisse promouvoir le commentaire ailleurs sur votre site.
- **Suivi de l'engagement** - enregistrez une note mémoire au sujet de l'utilisateur qui a écrit le commentaire afin que d'autres agents sachent qu'il a produit du contenu populaire.

### Ajustement

Le bon seuil dépend de la communauté. Surveillez l'[Historique d'exécution](#run-history) pendant quelques jours avec un seuil bas (5) pour voir la fréquence de déclenchement. Augmentez le seuil jusqu'à ce que le taux de déclenchement corresponde au rythme que vous souhaitez réellement.