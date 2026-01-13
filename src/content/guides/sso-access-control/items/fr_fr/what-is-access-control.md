Avec FastComments SSO Access Control, parfois appelé RBAC, les utilisateurs peuvent être restreints à n'accéder qu'à certaines pages, ou fils de commentaires. De plus, les utilisateurs ne peuvent `@mention` que d'autres utilisateurs du même groupe.

## En détail

Nous pouvons placer `Users` et, éventuellement, `Pages` dans des groupes.

Lorsque des `Users` sont placés dans des groupes, et que `Limit Comments by SSO User Groups` est activé dans les Paramètres du widget, alors les utilisateurs ne verront que les commentaires d'utilisateurs appartenant aux mêmes groupes et pourront uniquement `@mention` des utilisateurs du même groupe.

De plus, les `Pages` peuvent être placées dans des groupes, et les utilisateurs ne peuvent alors accéder qu'aux commentaires des pages auxquelles ils ont accès.

Nous appelons cela des groupes « User-Level » par opposition aux groupes « Page-Level ».

Lequel vous convient ?

#### Utilisez des groupes User-Level si...

- Vous voulez utiliser la même valeur `urlId` (URL de la page, ou ID de l'article), mais restreindre les commentaires par groupe.
- Par exemple, vous voulez avoir des groupes « New User » et « Veteran User », et ils ne devraient jamais voir les commentaires l'un de l'autre sur les mêmes pages.

#### Utilisez des groupes Page-Level si...

- Vos groupes ont des pages spécifiques.
- Par exemple, les utilisateurs du groupe « Public Pages » ne devraient jamais consulter les articles « Top Secret ».