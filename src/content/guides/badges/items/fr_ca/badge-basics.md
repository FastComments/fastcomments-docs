Les badges d'utilisateur FastComments sont configurés par les administrateurs disposant de l'autorisation `Customize Data`.

Ceci se fait via [Customize -> Badges](https://fastcomments.com/auth/my-account/configure-badges) dans votre tableau de bord d'administration.

Lorsqu'un utilisateur reçoit un badge, celui-ci s'affiche sur son profil et sur ses commentaires.

When adding a badge we can setup a `Display Label`, which is the name the user sees associated with the badge. For example, if we add a `Comment Count` badge
we probably don't want to show that technical name because it's very lame. We might call it `Super Member` or similar. Badges can also stack and replace each other, as we'll cover
later in this document.

Les badges ont aussi des seuils configurables.

Les badges peuvent être créés, puis désactivés ultérieurement en décochant `Enabled`. La désactivation d'un badge signifie qu'il ne sera plus attribué automatiquement, et n'apparaîtra plus dans le menu Attribuer un badge manuellement, mais les utilisateurs conserveront le badge.

### Types d'affichage des badges

Les badges peuvent être des images ou des badges textuels, qui prennent en charge un style de base (couleur du texte, couleur d'arrière-plan et couleur de bordure). Vous pouvez également styliser les badges via CSS.

Les badges image peuvent être des GIF pour afficher des badges animés.

### Astuce - Ne PAS supprimer les badges !

Les utilisateurs aiment les badges. Ils y tiennent beaucoup, même s'il s'agit d'un bug que vous avez ajouté par erreur et que vous voulez changer l'icône du badge.

Si nous avons appris quelque chose, c'est qu'il est extrêmement difficile d'ôter quelque chose aux utilisateurs. Supprimer un badge parce que vous, en tant que propriétaire du site, ne l'aimez plus ou voulez y apporter des modifications, peut entraîner une foule d'utilisateurs très en colère qui quittent soudainement votre site par frustration. Pour cette raison
`Delete` n'était même pas une option pendant les premiers mois suivant la sortie de cette fonctionnalité — cependant nous avons fini par devoir l'ajouter. Mais s'il vous plaît, utilisez la suppression avec prudence. Nous avons
vu de nombreux utilisateurs de longue date, présents depuis plusieurs années, se frustrer et quitter leurs communautés parce que les administrateurs ont décidé de supprimer un badge.

Si vous devez cesser d'utiliser un badge, nous vous suggérons simplement de le désactiver afin que les utilisateurs conservent leur badge.

### Recalcul des badges

When a badge is added or changed, the system will retroactively check anyone that interacted with your site to see if they should get the badge. This will be
visible in the Badges page in the admin dashboard, as a spinner will be shown instead of the number of users that have the badge. This is because the number of users
is being determined.

### Voir qui a un badge

In the Badges list each link has a `View Users` option to show the list of users which has earned or was manually awarded a badge.