Il est possible de bannir des utilisateurs utilisant certains fournisseurs d’e‑mail avec des caractères génériques.

Par exemple, si vous constatez que tous les commentaires provenant de **@bademail.com** sont du spam, vous pouvez simplement bannir tout le fournisseur d’e‑mail en saisissant "*@bademail.com" dans le champ e‑mail lors de l’ajout d’un utilisateur banni.

Notez le "*" avant le @ dans l’e‑mail.

### Subdomains

Un bannissement de domaine couvre également chaque sous‑domaine de ce domaine. Bannir `*@bademail.com` bannit aussi `someone@mail.bademail.com` et `someone@eu.mail.bademail.com`, il n’est donc pas nécessaire d’ajouter un bannissement séparé pour chaque sous‑domaine.

Si vous ne souhaitez bannir qu’un sous‑domaine spécifique, saisissez ce sous‑domaine à la place, par exemple `*@mail.bademail.com`. Ce bannissement n’affecte pas `someone@bademail.com`.

### Banning a Domain From a Comment

Vous n’avez pas besoin de saisir le motif vous‑même. Lorsque vous bannissez un utilisateur depuis un commentaire sur la page Modérer les commentaires, la boîte de dialogue de bannissement possède une case à cocher « Bannir tous les utilisateurs @domain » qui crée le même bannissement `*@domain` pour le domaine e‑mail du commentateur.

### Supported Patterns

La seule forme de caractère générique prise en charge est un `*` unique à la place de la partie nom complète, suivi de `@` et d’un domaine. Les autres formes sont rejetées lorsque vous essayez de les enregistrer :

- `*@*.bademail.com` n’est pas nécessaire, car `*@bademail.com` couvre déjà les sous‑domaine.
- `name*@bademail.com` et `*bademail.com` ne sont pas pris en charge.

---