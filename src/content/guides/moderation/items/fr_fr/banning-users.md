Il existe deux façons d'empêcher des utilisateurs de commenter sur votre site avec FastComments.

La première consiste, si vous connaissez déjà leur e‑mail, à la saisir sur la <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">utilisateurs bannis</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Cette page est accessible via Modérer les commentaires -> Utilisateurs bannis

Lorsque nous bannissons un utilisateur, nous pouvons choisir un type, soit Permanent soit Bannissement permanent masqué :

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

La deuxième façon de bannir un utilisateur est de cliquer sur le bouton de bannissement qui se trouve sur chaque commentaire de la page de modération des commentaires.

Lorsque vous cliquez sur le bouton de bannissement, des options s'affichent, vous permettant de préciser le type de bannissement et sa durée.

### Bannissements masqués

Un bannissement masqué est un type de bannissement qui donne l'impression que le commentaire ou le vote de l'utilisateur a été enregistré avec succès, alors qu'en réalité ce n'est pas le cas. Cela peut être souhaitable dans certaines situations.

### Bannissement par adresse IP

Sauf si un tenant choisit de se désengager, FastComments prend en charge le bannissement par adresse IP en stockant une version hachée de l'adresse IP du commentateur.