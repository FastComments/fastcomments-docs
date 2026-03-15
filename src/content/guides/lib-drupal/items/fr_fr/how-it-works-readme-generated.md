Lorsqu'un utilisateur visite une entité avec le champ FastComments activé :

1. Le widget JavaScript de FastComments est chargé depuis le CDN.
2. Si SSO est configuré, l'identité Drupal de l'utilisateur est transmise à FastComments.
3. Un mécanisme de secours `<noscript>` fournit des commentaires rendus côté serveur pour les utilisateurs sans JavaScript (uniquement dans les modes Live Comments et Streaming Chat).