Se déclenche lorsqu'un modérateur marque un commentaire comme examiné.

### Contexte que reçoit l'agent

- Le commentaire.
- L'**ID de l'utilisateur déclencheur** - le modérateur qui a examiné.
- Historique du fil / de l'utilisateur / contexte de la page optionnels, tels que configurés.

### Qui déclenche ceci

Une action d'un modérateur humain sur la page de modération, le widget de commentaires, ou via l'API.

### Utilisations courantes

- **Transfert d'audit** via [Webhooks](#webhooks-overview).
- **Écritures en mémoire** - enregistrer une note mémoire indiquant que ce commentaire a été examiné par un humain afin que d'autres agents ne le traitent pas en double.

### Remarques importantes

- "Examiné" est l'un des états de la file de modération suivis séparément de "approuvé" et "spam". Un commentaire peut être approuvé-et-examiné, approuvé-mais-pas-examiné, etc. Voir [Comment fonctionnent les approbations](/guide-moderation.html#moderation-approvals) dans le guide de modération.
- Ce déclencheur est très fréquent chez les locataires ayant de nombreux modérateurs. Abonnez-vous de façon sélective et prévoyez le budget en conséquence.

---