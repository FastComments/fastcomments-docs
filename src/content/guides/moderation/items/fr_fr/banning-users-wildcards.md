Il est possible de bannir des utilisateurs de certains fournisseurs d'e-mails en utilisant des caractères génériques.

Par exemple, si vous constatez que tous les commentaires provenant de **@bademail.com** sont du spam, vous pouvez simplement bannir
tout ce fournisseur d'e-mails en saisissant "*@bademail.com" dans le champ e-mail lors de l'ajout d'un utilisateur banni.

Notez le "*" avant le @ dans l'adresse e-mail.

### Sous-domaines

Un bannissement de domaine couvre également tous les sous-domaines de ce domaine. Bannir `*@bademail.com` bannit aussi
`someone@mail.bademail.com` et `someone@eu.mail.bademail.com`, il n'est donc pas nécessaire d'ajouter un bannissement séparé pour chaque sous-domaine.

Si vous souhaitez uniquement bannir un sous-domaine spécifique, saisissez plutôt ce sous-domaine, par exemple `*@mail.bademail.com`. Ce bannissement
n'affecte pas `someone@bademail.com`.

### Bannir un domaine depuis un commentaire

Vous n'avez pas besoin de saisir le motif vous-même. Lorsque vous bannissez un utilisateur depuis un commentaire sur la page Modérer les commentaires, la boîte de dialogue
de bannissement comporte une case à cocher « Ban All @domain Users » qui crée le même bannissement `*@domain` pour le domaine e-mail du commentateur.

### Motifs pris en charge

La seule forme de caractère générique prise en charge est un seul `*` à la place de la partie nom, suivi de `@` et d'un domaine. Les autres formes
sont rejetées lorsque vous essayez de les enregistrer :

- `*@*.bademail.com` n'est pas nécessaire, car `*@bademail.com` couvre déjà les sous-domaines.
- `name*@bademail.com` et `*bademail.com` ne sont pas pris en charge.