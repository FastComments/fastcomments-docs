---
Les modérateurs peuvent être placés dans des groupes pour modérer différentes pages ou catégories de contenu.

Lorsqu'un modérateur appartient à un ou plusieurs groupes, il ne verra que les commentaires de ces groupes sur la page de modération des commentaires.

Par exemple, supposons que nous gérions un site qui affiche des vidéos par catégorie. Nous pourrions vouloir avoir des modérateurs différents pour les vidéos de chats, chiens et perroquets, donc [ajoutons ces groupes](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

Lorsque nous ajoutons un modérateur, nous avons désormais la possibilité de sélectionner un ou plusieurs groupes auxquels le modérateur appartiendra :

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

Enfin, les commentaires doivent être liés à un ou plusieurs groupes afin que les modérateurs appropriés puissent les voir.

Cela peut être configuré en [ajoutant quelques groupes](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) puis en spécifiant les identifiants `Moderation Group` correspondants dans le widget de commentaires,
[comme indiqué ici](/guide-customizations-and-configuration.html#moderation-group-ids).

---