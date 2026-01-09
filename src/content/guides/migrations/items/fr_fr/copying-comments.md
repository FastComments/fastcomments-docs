Si des données doivent être déplacées, FastComments fournit un outil en libre-service pour déplacer les commentaires entre des pages et des articles.

Voici à quoi ressemble le formulaire de copie de commentaires :

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Remplir les champs « From »

Pour déterminer d'où déplacer les commentaires, il suffit de connaître le `URL ID` source.

Si vous ne transmettez pas de valeur pour `urlId` dans la configuration du widget de commentaires, alors il s'agira d'une version "propre" de l'URL de la page.

Vous pouvez voir quelles valeurs de `URL ID` sont associées à vos commentaires en les exportant.

### Remplir les champs « To »

Pour décider où déplacer les commentaires, nous avons besoin du `URL ID` cible et de l'`URL`.

Le `URL ID` sera le compartiment (bucket) dans lequel le commentaire sera placé. Le champ `URL` est utilisé afin que vous puissiez naviguer directement vers le commentaire depuis les e-mails et les outils de modération.

#### WordPress

Si vous utilisez WordPress, vous saisiriez par exemple les ID d'article dans les champs To/From `URL ID` de l'outil de migration, plutôt qu'une URL.