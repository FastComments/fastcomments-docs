Chaque instance du widget de commentaires est isolée. Pour cette raison, FastComments prend naturellement en charge plus d'une instance par page, ou plusieurs instances pointant vers le même fil de discussion.

Dans le cas de la bibliothèque VanillaJS par exemple, vous devez simplement lier le widget de commentaires à différents nœuds DOM. Si vous souhaitez simplement mettre à jour le fil de discussion actuel sur la page, consultez [Changer de fil de commentaires sans recharger la page](guide-customizations-and-configuration.html#switching-comment-threads);

### Synchronisation de l'état d'authentification entre plusieurs instances

Prenons l'exemple d'une application monopage personnalisée qui est une liste de questions fréquemment posées avec leur propre fil de commentaires.

Dans ce cas, nous avons plusieurs instances de FastComments dans le DOM en même temps.

C'est correct, mais cela pose certains défis pour l'expérience utilisateur.

Considérez ce flux :

1. L'utilisateur visite la page avec une liste de questions, chacune avec son propre widget de commentaires.
2. L'utilisateur entre son nom d'utilisateur et son courriel et laisse une question sur l'un des fils.
3. Il voit un autre élément FAQ sur lequel il a une question.
4. Il va commenter à nouveau. Doit-il entrer son courriel et son nom d'utilisateur à nouveau?

Dans ce cas, FastComments gère la synchronisation de l'état d'authentification entre les instances du widget pour vous. À l'étape quatre, l'utilisateur sera déjà temporairement authentifié puisqu'il a entré son nom d'utilisateur et son courriel sur la même page.
