[Val Town](https://val.town) exécute TypeScript sur Deno, donc un val est un vrai serveur. Cela en fait un bon choix pour FastComments : le widget est une balise script sur la page, et tout ce qui nécessite un secret, comme le SSO sécurisé ou la vérification d’un webhook, peut s’exécuter côté serveur dans le même val.

Ce guide couvre l’ajout du widget de commentaires à un val HTTP, l’affichage du nombre de commentaires sur une page d’index, la connexion des utilisateurs avec le compte Val Town qu’ils possèdent déjà, et la réception des webhooks de commentaires.

Vous n’avez pas besoin de compte pour l’essayer. Les exemples utilisent `tenantId: "demo"`, un bac à sable partagé, et l’étape 2 explique comment passer au vôtre.