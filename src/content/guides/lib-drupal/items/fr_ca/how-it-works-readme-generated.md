---
Lorsqu'un utilisateur visite une entité pour laquelle le champ FastComments est activé :

1. Le widget JavaScript FastComments est chargé depuis le CDN.
2. Si SSO est configuré, l'identité Drupal de l'utilisateur est transmise à FastComments.
3. Un repli `<noscript>` fournit des commentaires rendus côté serveur pour les utilisateurs sans JavaScript (uniquement en modes Live Comments et Streaming Chat).
---