Avec FastComments SSO Access Control, parfois appelé RBAC, les utilisateurs peuvent se voir limiter l'accès à certaines pages ou fils de commentaires. De plus, les utilisateurs ne peuvent `@mention` que des membres du même groupe.

## En détail

Nous pouvons placer des `Users` et, facultativement, des `Pages` dans des groupes.

Lorsque des `Users` sont placés dans des groupes, et que `Limit Comments by SSO User Groups` est activé dans les Paramètres du widget, alors les utilisateurs ne verront que les commentaires d'utilisateurs appartenant aux mêmes groupes et ne pourront `@mention` que des utilisateurs du même groupe.

De plus, des `Pages` peuvent être placées dans des groupes, et alors les utilisateurs ne pourront accéder qu'aux commentaires des pages auxquelles ils ont accès.

Nous appelons cela des groupes de "User-Level" par opposition aux groupes de "Page-Level".

Alors, lequel vous convient le mieux ?

#### Use User-Level Groups if...

- Vous souhaitez utiliser la même valeur `urlId` (URL de la page ou ID d'article), mais restreindre les commentaires par groupe.
- Par exemple, vous souhaitez avoir des groupes "New User" et "Veteran User", et ils ne devraient jamais voir les commentaires des autres sur les mêmes pages.

#### Use Page-Level Groups if...

- Vos groupes ont des pages spécifiques.
- Par exemple, les utilisateurs du groupe "Public Pages" ne devraient jamais consulter les articles "Top Secret".