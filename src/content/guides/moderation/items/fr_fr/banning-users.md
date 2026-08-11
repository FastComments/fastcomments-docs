---
Il existe deux façons d’interdire aux utilisateurs de commenter sur votre site avec FastComments.

La première, si vous connaissez déjà leur e‑mail, vous pouvez le saisir sur la page <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utilisateurs bannis</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Liste des utilisateurs bannis sous Modérer les commentaires, avec les adresses e‑mail bannies et un bouton pour ajouter un nouveau bannissement'; title='Page des utilisateurs bannis' app-screenshot-end]

Cette page est accessible via Modérer les commentaires -> Utilisateurs bannis

Lorsque nous voulons bannir un utilisateur, nous pouvons choisir un type, soit Permanent ou Permanent Shadow Ban :

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Formulaire de nouveau bannissement avec un champ e‑mail et un choix de type de bannissement : Permanent ou Permanent Shadow Ban'; title='Bannir un utilisateur' app-screenshot-end]

La deuxième façon de bannir un utilisateur consiste à cliquer sur le bouton de bannissement placé sur chaque commentaire de la page de modération des commentaires.

Lorsque nous cliquons sur le bouton de bannissement, vous verrez plusieurs options, où nous pouvons spécifier le type de bannissement et la durée.

### Alias d'e‑mail

Lors du bannissement d'un utilisateur par e‑mail, FastComments ignore automatiquement les alias `+`. Par exemple, bannir `user+alias@gmail.com` bannira également `user@gmail.com` ainsi que toute autre variante `+` de cette adresse, comme `user+other@gmail.com`.

### Bannissements fantômes

Un bannissement fantôme est un type de bannissement qui donne l'impression que le commentaire ou le vote de l'utilisateur a été enregistré avec succès, alors qu'en réalité ce n'est pas le cas. Cela peut être souhaitable dans certaines situations.

### Bannissement par adresse IP

À moins qu'un locataire ne souhaite se désinscrire, FastComments prend en charge le bannissement par IP en stockant une version hachée de l'adresse IP du commentateur.
---