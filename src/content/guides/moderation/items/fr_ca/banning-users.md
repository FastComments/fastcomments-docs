Il y a deux façons de bannir des utilisateurs pour qu'ils ne commentent plus sur votre site avec FastComments.

La première consiste, si vous connaissez déjà leur courriel, à l'entrer sur la page <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utilisateurs bannis</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='La page des utilisateurs bannis' app-screenshot-end]

Cette page est accessible via Modérer les commentaires -> Utilisateurs bannis

Lorsque nous voulons bannir un utilisateur, nous pouvons choisir un type, soit Permanent, soit Bannissement furtif permanent :

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Bannir un utilisateur' app-screenshot-end]

La deuxième façon de bannir un utilisateur est de cliquer sur le bouton de bannissement qui est placé sur chaque commentaire de la page de modération des commentaires.

Lorsque nous cliquons sur le bouton de bannissement, des options s'affichent, où nous pouvons préciser le type de bannissement et sa durée.

### Alias de courriel

Lors de la mise sur liste noire d'un utilisateur par courriel, FastComments ignore automatiquement les alias `+`. Par exemple, bannir `user+alias@gmail.com` bannira aussi `user@gmail.com` et toute autre variation `+` de cette adresse, comme `user+other@gmail.com`.

### Bannissements furtifs

Un bannissement furtif est un type de bannissement qui fait apparaître que le commentaire ou le vote de l'utilisateur a été enregistré avec succès, alors qu'en réalité ce n'est pas le cas. Cela peut être souhaitable dans certaines situations.

### Bannissement par adresse IP

Sauf si un locataire choisit de se retirer, FastComments prend en charge le bannissement par adresse IP en stockant une version hachée de l'adresse IP du commentateur.