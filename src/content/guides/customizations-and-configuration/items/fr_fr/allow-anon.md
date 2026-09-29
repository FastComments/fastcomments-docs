---
Par défaut, FastComments exigera une adresse e‑mail pour commenter. Elle n’a pas besoin d’être valide, cependant tant que l’utilisateur ne clique pas sur le lien qui lui est envoyé,
son commentaire affichera l’étiquette « Commentaire non vérifié ».

Cependant, nous pouvons supprimer l’exigence d’e‑mail. Le champ de saisie de l’e‑mail sera toujours affiché, mais il ne sera plus obligatoire.

Cela peut être configuré via l’interface de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.allow-anonymous-comments'; alt='Option de commentaires anonymes dans l\'interface de personnalisation du widget, qui rend le champ e‑mail facultatif'; title='Activation des commentaires anonymes' app-screenshot-end]

Un nom d’utilisateur est toujours requis. Pour supprimer également cette étape, vous pouvez
[definir un nom d’utilisateur par défaut](/guide-customizations-and-configuration.html#default-username) que tout le monde partage, ou faire en sorte que FastComments
[génère un nom d’utilisateur unique](/guide-customizations-and-configuration.html#auto-generate-username) pour chaque visiteur.
---