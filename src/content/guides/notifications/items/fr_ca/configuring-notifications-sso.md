Pour SSO il y a la configuration suivante à prendre en compte pour les notifications :

- Si l'utilisateur a choisi de recevoir des notifications.
  - Cela se fait en définissant le drapeau `optedInNotifications` sur `true` ou `false` dans l'objet `SSOUser`.
  - Cela peut être défini via l'API.
  - De plus, si vous fournissez une valeur pour ce drapeau dans la charge utile, elle sera automatiquement mise à jour lorsque l'utilisateur chargera un fil de commentaires.
- Si l'utilisateur a choisi de recevoir des notifications **d'abonnement**.
  - Cela se fait en définissant le drapeau `optedInSubscriptionNotifications` sur `true` ou `false` dans l'objet `SSOUser`.
  - Cela peut être défini via l'API.
  - De plus, si vous fournissez une valeur pour ce drapeau dans la charge utile, elle sera automatiquement mise à jour lorsque l'utilisateur chargera un fil de commentaires.
- Définir leur adresse courriel.
  - Si elle n'est pas présente, nous ne pouvons pas envoyer de notifications par courriel.
- S'il faut désactiver les liens de désabonnement dans les courriels.
  - Cela se fait via le drapeau `disableUnsubscribeLinks` dans l'objet `Tenant`.
  - Cela peut être défini via l'API.
- S'il faut utiliser un lien de désabonnement personnalisé.
  - Cela se fait via la propriété `footerUnsubscribeURL` dans l'objet `DomainConfig`.
  - Cela peut être défini via l'API.
  - Vous pouvez également envisager de définir les en-têtes de désabonnement pertinents via `emailHeaders` dans le même objet.