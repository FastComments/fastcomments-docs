Lorsqu'on modère et visualise des fils de commentaires, il est souhaitable de pouvoir accéder directement à un fil pour obtenir du contexte pendant la modération.

Cela signifie que le parcours de l'utilisateur commence sur la page de modération des commentaires, et devrait ensuite passer d'un commentaire individuel vers
la page contenant ce commentaire, attendre que cette page se charge, attendre que les commentaires se chargent, puis faire défiler jusqu'à ce commentaire.

Cependant, FastComments offre une méthode plus rapide. Sur la page de modération des commentaires, à côté de chaque commentaire, il y a un bouton "Voir le commentaire" en bas à droite.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

Si ce commentaire a des réponses, le texte du bouton indiquera plutôt le nombre de réponses, mais cliquer dessus effectue la même action.

Ce bouton vous mènera à la **Visionneuse de fil de commentaires**.

La Visionneuse de fil de commentaires est une petite application à chargement rapide hébergée par FastComments qui affiche le fil de commentaires de la page sur laquelle
se trouve le commentaire, et fait défiler jusqu'à ce commentaire.

Cela permet aux modérateurs de rassembler rapidement le contexte dont ils ont besoin, sans avoir à attendre le chargement d'une autre page.

---