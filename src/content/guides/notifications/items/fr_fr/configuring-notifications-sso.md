Pour SSO, il y a la configuration suivante à prendre en compte pour les notifications :

- Savoir si l'utilisateur a choisi de recevoir des notifications.
  - Cela se fait en définissant le flag `optedInNotifications` sur `true` ou `false` dans l'objet `SSOUser`.
  - Cela peut être défini via l'API.
  - De plus, si vous passez une valeur pour ce flag dans la charge utile, elle sera automatiquement mise à jour lorsque l'utilisateur charge un fil de commentaires.
- Savoir si l'utilisateur a choisi de recevoir des notifications **d'** **abonnement**.
  - Cela se fait en définissant le flag `optedInSubscriptionNotifications` sur `true` ou `false` dans l'objet `SSOUser`.
  - Cela peut être défini via l'API.
  - De plus, si vous passez une valeur pour ce flag dans la charge utile, elle sera automatiquement mise à jour lorsque l'utilisateur charge un fil de commentaires.
- Définir leur adresse email.
  - S'il est absent, nous ne pouvons pas envoyer de notifications par email.
- Savoir s'il faut désactiver les liens de désabonnement dans les emails.
  - Cela se fait via le flag `disableUnsubscribeLinks` dans l'objet `Tenant`.
  - Cela peut être défini via l'API.
- Savoir s'il faut utiliser un lien de désabonnement personnalisé.
  - Cela se fait via la propriété `footerUnsubscribeURL` sur l'objet `DomainConfig`.
  - Cela peut être défini via l'API.
  - Vous pouvez également envisager de définir les en-têtes de désabonnement pertinents via `emailHeaders` dans le même objet.

---