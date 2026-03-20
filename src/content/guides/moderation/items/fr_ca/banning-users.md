Il existe deux façons d'empêcher des utilisateurs de commenter sur votre site avec FastComments.

La première consiste, si vous connaissez déjà leur adresse courriel, à la saisir sur la <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">page des utilisateurs bannis</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Cette page est accessible via Modération des commentaires -> Utilisateurs bannis

Lorsque nous procédons au bannissement d'un utilisateur, nous pouvons choisir un type : Permanent ou shadow ban permanent :

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

La deuxième façon de bannir un utilisateur est de cliquer sur le bouton de bannissement qui se trouve sur chaque commentaire de la page de modération des commentaires.

Lorsque vous cliquez sur le bouton de bannissement, des options s'affichent, vous permettant de préciser le type de bannissement et sa durée.

### Shadow Bans

Un shadow-ban est un type de bannissement qui donne l'impression que le commentaire ou le vote de l'utilisateur a été enregistré avec succès, alors qu'en réalité ce n'est pas le cas. Cela peut être souhaitable dans certaines situations.

### Bannissement par adresse IP

À moins qu'un locataire ne souhaite se retirer, FastComments prend en charge le bannissement par adresse IP en stockant une version hachée de l'adresse IP du commentateur.