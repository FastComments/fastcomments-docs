Au cas où des données doivent être déplacées, FastComments fournit un outil en libre-service pour déplacer des commentaires
entre des pages et des articles.

Voici à quoi ressemble le formulaire de copie de commentaires :

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='Le formulaire de copie de commentaire' app-screenshot-end]

### Remplir les champs "From"

Pour déterminer d'où déplacer les commentaires, nous devons simplement connaître le `URL ID` source.

Si vous ne transmettez pas de valeur pour `urlId` dans la configuration du widget de commentaires, il s'agira alors d'une version "propre" de l'URL de la page.

Vous pouvez voir quelles valeurs vos commentaires ont pour `URL ID` en les exportant.

### Remplir les champs "To"

Pour déterminer où déplacer les commentaires, nous devons connaître le `URL ID` cible et l'`URL`.

Le `URL ID` sera le bucket dans lequel le commentaire est placé. Le champ `URL` est utilisé afin que vous puissiez naviguer directement
vers le commentaire à partir des courriels et des outils de modération.

#### WordPress

Si vous utilisez WordPress, vous entreriez par exemple les ID d'article dans les champs To/From `URL ID` de l'outil de migration,
plutôt que l'URL.