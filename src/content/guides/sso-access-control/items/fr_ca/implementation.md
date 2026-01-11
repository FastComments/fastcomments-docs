#### Mentionner des utilisateurs d'autres groupes

Si deux utilisateurs appartiennent à deux ensembles de groupes différents, et qu'il n'y a aucune intersection, ils ne pourront pas se `@mention`ner.

Si un utilisateur tape manuellement un `@mention` et soumet son commentaire, il restera sous forme de texte brut. L'autre utilisateur ne sera pas étiqueté.

#### Maintien des groupes

`Groups` sont définis en utilisant respectivement les ressources API `Pages` et `SSOUsers`.

L'API `Pages` peut être appelée pour définir l'ensemble des groupes autorisés à accéder à la page. Par défaut, tous les groupes, ainsi que les utilisateurs qui n'appartiennent pas à un groupe, ont accès.

De même, l'API `SSOUsers` peut être appelée pour définir les groupes associés à chaque utilisateur.

Pour les deux ressources, il n'y a aucune limitation quant au moment où les groupes peuvent être définis ou mis à jour.

Si l'on souhaite uniquement empêcher les utilisateurs de se `@mention`ner mutuellement, alors il n'est pas nécessaire de prendre en compte les `Pages`.

##### Remarque !

Defining and updating the SSO user groups does not require using the API, and can instead be updated automatically by defining the group ids in the SSO payload passed to the comment widget. However, for large lists of groups, this is not recommended as the user would have to submit this payload for every page load.