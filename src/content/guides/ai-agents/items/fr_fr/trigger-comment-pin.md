Se déclenche lorsqu'un commentaire est épinglé.

### Contexte reçu par l'agent

- Le commentaire épinglé.
- Historique facultatif du fil / de l'utilisateur / contexte de la page tel que configuré.

### Qui déclenche cet événement

- Un modérateur cliquant sur l'action d'épinglage sur la page de modération ou le widget de commentaire.
- Un agent appelant [`pin_comment`](#tools-overview).

Prévention des boucles : les événements d'épinglage provenant d'un agent n'entraînent l'exécution d'aucun déclencheur d'agent. Un épinglage effectué par un agent court-circuite toute diffusion d'événements aux agents pour cet événement, pas seulement l'agent à l'origine.

### Remarque sur la paire

Les événements d'épinglage et de désépinglage sont des déclencheurs distincts. Abonnez-vous aux deux si vous souhaitez un comportement symétrique. Voir [Déclencheur : Commentaire désépinglé](#trigger-comment-unpin).

---