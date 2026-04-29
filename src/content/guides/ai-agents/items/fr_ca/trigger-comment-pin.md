Se déclenche lorsqu'un commentaire est épinglé.

### Contexte que reçoit l'agent

- Le commentaire épinglé.
- Contexte optionnel de la discussion / historique de l'utilisateur / page, selon la configuration.

### Qui déclenche ceci

- Un modérateur cliquant sur l'action d'épinglage sur la page de modération ou le widget de commentaire.
- Un agent appelant [`pin_comment`](#tools-overview).

Prévention des boucles : les événements d'épinglage provenant d'un agent ne déclenchent aucun déclencheur d'agent. Un épinglage effectué par un agent court-circuite tout envoi d'événements aux agents pour cet événement, pas seulement à l'agent d'origine.

### Remarque sur la paire

Les événements d'épinglage et de désépinglage sont des déclencheurs distincts. Abonnez-vous aux deux si vous voulez un comportement symétrique. Voir [Déclencheur : Commentaire désépinglé](#trigger-comment-unpin).

---