Les badges d'utilisateur FastComments sont configurés par les administrateurs disposant de l'autorisation `Customize Data`.

Cela se fait via [Personnaliser -> Badges](https://fastcomments.com/auth/my-account/configure-badges) dans votre tableau de bord administrateur.

Lorsqu'un utilisateur se voit attribuer un badge, celui-ci s'affiche sur son profil et sur ses commentaires.

Lors de l'ajout d'un badge, nous pouvons configurer un `Display Label`, qui est le nom que l'utilisateur voit associé au badge. Par exemple, si nous ajoutons un badge `Comment Count` nous ne voudrons probablement pas afficher ce nom technique car il est assez terne. Nous pourrions l'appeler `Super Member` ou similaire. Les badges peuvent également se cumuler et se remplacer les uns les autres, comme nous le verrons plus loin dans ce document.

Les badges ont aussi des seuils configurables.

Les badges peuvent être créés, puis désactivés ultérieurement en décochant `Enabled`. Désactiver un badge signifie qu'il ne sera plus attribué automatiquement et n'apparaîtra plus dans le menu Attribuer un badge manuellement, mais les utilisateurs conserveront le badge.

### Types d'affichage des badges

Les badges peuvent être des images ou des badges texte, qui prennent en charge quelques styles de base (couleur du texte, couleur de fond et couleur de bordure). Vous pouvez également styliser les badges via CSS.

Les badges image peuvent être des GIF pour afficher des badges animés.

### Astuce - Ne supprimez pas les badges !

Les utilisateurs adorent les badges. Ils y tiennent beaucoup, même si c'est un badge ajouté par erreur ou un bug, et vous voulez changer l'icône du badge.

Si nous avons appris quelque chose, c'est qu'il est extrêmement difficile de retirer quelque chose aux utilisateurs. Supprimer un badge parce que vous, en tant que propriétaire du site, ne l'aimez plus ou souhaitez le modifier, peut entraîner une foule d'utilisateurs très en colère qui quitteraient soudainement votre site par frustration. Pour cette raison, la fonction `Delete` n'était même pas disponible pendant les premiers mois suivant la sortie de cette fonctionnalité - cependant nous avons finalement dû l'ajouter. Mais s'il vous plaît, utilisez la fonction `Delete` avec prudence. Nous avons vu de nombreux utilisateurs de longue date, présents depuis plusieurs années, se frustrer et quitter leurs communautés parce que des administrateurs ont décidé de supprimer un badge.

Si vous devez cesser d'utiliser un badge, nous vous suggérons simplement de le désactiver afin que les utilisateurs conservent leur badge.

### Recalcul des badges

Lorsqu'un badge est ajouté ou modifié, le système vérifiera rétroactivement toutes les personnes qui ont interagi avec votre site pour déterminer si elles doivent obtenir le badge. Cela sera visible sur la page des badges dans le tableau de bord administrateur, car un spinner s'affichera à la place du nombre d'utilisateurs qui possèdent le badge. Cela signifie que le nombre d'utilisateurs est en cours de détermination.

### Voir qui a un badge

Dans la liste des Badges, chaque lien propose une option `View Users` pour afficher la liste des utilisateurs qui ont gagné ou se sont vu attribuer manuellement un badge.