Dans le cas où des données doivent être déplacées, FastComments propose un outil en libre-service permettant de déplacer les commentaires entre les pages et les articles.

Voici à quoi ressemble le formulaire de copie de commentaires :

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Formulaire de copie de commentaires avec le champ ID d\'URL d\'origine et les champs ID d\'URL de destination et URL'; title='Formulaire de copie de commentaires' app-screenshot-end]

### Remplir les champs « From »

Pour déterminer d'où déplacer les commentaires, nous devons simplement connaître le `URL ID` source.

Si vous ne transmettez pas de valeur pour `urlId` dans la configuration du widget de commentaires, alors il s'agira d'une version « clean » de l'URL de la page.

Vous pouvez voir quelles valeurs vos commentaires ont pour le `URL ID` en les exportant.

### Remplir les champs « To »

Pour déterminer où déplacer les commentaires, nous devons connaître le `URL ID` cible et l'`URL`.

Le `URL ID` sera le compartiment dans lequel le commentaire sera placé. Le champ `URL` est utilisé afin que vous puissiez accéder directement au commentaire depuis les e‑mails et les outils de modération.

#### WordPress

Si vous utilisez WordPress, vous saisiriez par exemple les ID d'articles dans les champs `URL ID` To/From de l'outil de migration, plutôt qu'une URL.