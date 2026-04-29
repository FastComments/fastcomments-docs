Se déclenche lorsqu'un modérateur marque un commentaire comme révisé.

### Contexte reçu par l'agent

- Le commentaire.
- L'**ID de l'utilisateur déclencheur** - le modérateur qui a révisé.
- Historique optionnel du fil / de l'utilisateur / contexte de la page tel que configuré.

### Qui déclenche cet événement

Une action d'un modérateur humain sur la page de modération, le widget de commentaires ou via l'API.

### Utilisations courantes

- **Transfert d'audit** via [Webhooks](#webhooks-overview).
- **Écritures mémoire** - enregistrer une note en mémoire indiquant que ce commentaire a été révisé par un humain afin que d'autres agents ne le traitent pas en double.

### À noter

- "Révisé" fait partie des états de la file de modération suivis séparément de "approuvé" et "pourriel". Un commentaire peut être approuvé-et-révisé, approuvé-mais-non-révisé, etc. Voir [Comment fonctionnent les approbations](/guide-moderation.html#moderation-approvals) dans le guide de modération.
- Ce déclencheur est fréquent sur les locataires ayant de nombreux modérateurs. Abonnez-vous sélectivement et prévoyez un budget en conséquence.

---