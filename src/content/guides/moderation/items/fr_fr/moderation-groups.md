Les modérateurs peuvent être placés dans des groupes pour modérer différentes pages ou catégories de contenu.

Lorsqu'un modérateur appartient à un ou plusieurs groupes, il ne verra que les commentaires de ces groupes dans la page Modérer les commentaires.

Par exemple, disons que nous gérons un site qui affiche des vidéos par catégorie. Nous pouvons vouloir avoir différents modérateurs pour les vidéos de chats, de chiens et de perroquets, donc [ajoutons ces groupes](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Liste des groupes de modération avec les groupes Chat, Chien et Perroquet créés pour chaque catégorie de vidéos'; title='La page des groupes de modération' app-screenshot-end]

Lorsque nous ajoutons un modérateur, nous avons maintenant la possibilité de sélectionner un ou plusieurs groupes auxquels le modérateur appartiendra :

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Formulaire d\'ajout d\'un modérateur avec le sélecteur de groupe utilisé pour affecter le modérateur à un ou plusieurs groupes'; title='Ajout d\'un modérateur et sélection d\'un groupe' app-screenshot-end]

Enfin, les commentaires doivent être liés à un ou plusieurs groupes afin que les bons modérateurs les voient.

Cela peut être configuré en [ajoutant des groupes](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) puis en spécifiant les identifiants `Moderation Group` correspondants dans le widget de commentaires,
[comme indiqué ici](/guide-customizations-and-configuration.html#moderation-group-ids).