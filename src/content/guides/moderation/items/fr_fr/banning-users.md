Il existe deux façons d'empêcher des utilisateurs de commenter sur votre site avec FastComments.

La première, si vous connaissez déjà leur adresse e-mail, consiste à la saisir sur la <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utilisateurs bannis</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Cette page est accessible via Modérer les commentaires -> Utilisateurs bannis

Lorsque nous allons bannir un utilisateur, nous pouvons choisir un type : soit Permanent, soit Shadow-ban permanent :

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

La deuxième façon d'interdire un utilisateur est de cliquer sur le bouton de bannissement placé sur chaque commentaire de la page Modération des commentaires.

Lorsque vous cliquez sur le bouton de bannissement, des options vous seront présentées, où vous pouvez spécifier le type et la durée de l'interdiction.

### Alias d'e-mail

Lorsqu'on bannit un utilisateur par e-mail, FastComments ignore automatiquement les alias `+`. Par exemple, bannir `user+alias@gmail.com` bannira également `user@gmail.com` et toute autre variation avec `+` de cette adresse, comme `user+other@gmail.com`.

### Bannissements masqués

Un bannissement masqué est un type d'interdiction qui donne l'impression que le commentaire ou le vote de l'utilisateur a été enregistré avec succès, alors qu'en réalité ce n'était pas le cas. Cela peut être souhaitable dans certaines situations.

### Bannissement via l'adresse IP

Sauf si un tenant souhaite se désinscrire, FastComments prend en charge le bannissement par IP en stockant une version hachée de l'adresse IP du commentateur.