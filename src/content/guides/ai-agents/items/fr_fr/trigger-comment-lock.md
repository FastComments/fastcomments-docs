Se déclenche lorsqu'un commentaire est verrouillé.

### Contexte reçu par l'agent

- Le commentaire verrouillé.
- Historique facultatif du fil / de l'utilisateur / contexte de la page selon la configuration.

### Qui déclenche cet événement

- Un modérateur utilisant l'action de verrouillage sur la page de modération ou le widget de commentaires.

### Utilisations courantes

- **Notifier les réviseurs** - un événement de verrouillage suit souvent un fil animé ; un webhook vers votre canal Slack de modération peut permettre aux humains de prendre la suite.
- **Application d'une période de refroidissement** - planifiez un [déclencheur différé](#trigger-deferred-delay) sur un agent séparé qui, 24 heures après le verrouillage, évalue s'il faut déverrouiller.

### Événement correspondant

Voir [Déclencheur : Commentaire déverrouillé](#trigger-comment-unlock) pour le déclencheur miroir.

---