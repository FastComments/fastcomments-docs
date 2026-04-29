Se déclenche lorsqu'un commentaire est verrouillé.

### Contexte que l'agent reçoit

- Le commentaire verrouillé.
- Historique optionnel du fil / de l'utilisateur / contexte de la page tel que configuré.

### Qui le déclenche

- Un modérateur utilisant l'action de verrouillage sur la page de modération ou le widget de commentaire.

### Utilisations courantes

- **Notifier les réviseurs** - un événement de verrouillage suit souvent un fil houleux ; un webhook vers votre canal Slack de modération permet aux humains de prendre la relève.
- **Faire respecter le délai de refroidissement** - planifiez un [déclencheur différé](#trigger-deferred-delay) sur un agent séparé qui, 24 heures après le verrouillage, évalue s'il faut déverrouiller.

### Événement correspondant

Voir [Déclencheur : commentaire déverrouillé](#trigger-comment-unlock) pour le déclencheur miroir.

---