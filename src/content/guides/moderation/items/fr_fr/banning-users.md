Il existe deux manières d'empêcher des utilisateurs de commenter sur votre site avec FastComments.

La première consiste, si vous connaissez déjà leur adresse e-mail, à l'entrer sur la <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utilisateurs bannis</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Cette page est accessible via Modérer les commentaires -> Utilisateurs bannis

Lorsque nous souhaitons bannir un utilisateur, nous pouvons choisir un type : soit Permanent soit Shadow ban permanent :

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

La deuxième façon de bannir un utilisateur consiste à cliquer sur le bouton de bannissement placé sur chaque commentaire de la page de modération des commentaires.

Lorsque nous cliquons sur le bouton de bannissement, des options s'affichent, nous permettant de préciser le type et la durée du bannissement.

### Bannissements furtifs

Un bannissement furtif est un type de bannissement qui fait croire que le commentaire ou le vote de l'utilisateur a été enregistré avec succès, alors qu'en réalité ce n'est pas le cas. Cela peut être souhaitable dans certaines situations.

### Bannissement par adresse IP

Sauf si un locataire souhaite se retirer, FastComments prend en charge le bannissement via l'adresse IP en stockant une version hachée de l'adresse IP du commentateur.