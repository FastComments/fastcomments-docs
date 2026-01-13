#### Mentionner des utilisateurs dans d'autres groupes

Si deux utilisateurs appartiennent à deux ensembles de groupes différents, et qu'il n'y a aucune intersection, ils ne pourront pas se `@mention` mutuellement.

Si un utilisateur saisit manuellement un `@mention` et soumet son commentaire, cela restera du texte brut. L'autre utilisateur ne sera pas mentionné.

#### Gestion des groupes

Les `Groups` sont définis en utilisant respectivement les ressources API `Pages` et `SSOUsers`.

L'API `Pages` peut être invoquée pour définir l'ensemble des groupes autorisés à accéder à la page. Par défaut, tous les groupes, ainsi que les utilisateurs qui n'appartiennent à aucun groupe, ont accès.

De même, l'API `SSOUsers` peut être invoquée pour définir les groupes associés à chaque utilisateur.

Pour les deux ressources, il n'y a pas de limitation quant au moment où les groupes peuvent être définis ou mis à jour.

Si l'on souhaite uniquement empêcher que les utilisateurs se `@mention` mutuellement, alors les `Pages` n'ont pas à être prises en compte.

##### Remarque !

La définition et la mise à jour des groupes d'utilisateurs SSO ne nécessitent pas l'utilisation de l'API, et peuvent à la place être mises à jour automatiquement en définissant les ids de groupe dans le payload SSO transmis au widget de commentaires. Cependant, pour de longues listes de groupes, cela n'est pas recommandé car l'utilisateur devrait soumettre ce payload à chaque chargement de page.