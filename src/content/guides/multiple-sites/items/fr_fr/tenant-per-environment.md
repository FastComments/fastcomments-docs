Il est courant d'avoir un sous-tenant par environnement de test ou de développement avec FastComments. Chaque tenant aurait sa propre configuration, ses propres données, et ses propres clés API. La configuration, les données et les utilisateurs ne peuvent pas être partagés entre les tenants.
Tout est isolé. Cependant, les super-admins du tenant parent peuvent se faire passer pour des utilisateurs des tenants enfants.

Il existe deux approches :

- Le tenant principal est pour la production, et les sous-tenants sont pour les environnements de test.
- Le tenant principal est simplement pour la facturation, et chaque sous-tenant est pour prod, test, et ainsi de suite.

La première est généralement plus facile à comprendre pour les utilisateurs, mais cela peut dépendre de votre organisation.

Les tenants peuvent être créés [ici](https://eu.fastcomments.com/auth/my-account/tenants) si vous avez le package. C'est aussi là que les super-admins se feraient passer pour des utilisateurs. Les tenants peuvent aussi être créés via l'API pour des configurations plus personnalisées/automatisées.

Quelle que soit l'approche choisie, vous devrez ajouter les modérateurs et utilisateurs qui veulent voir les données de production dans le tenant "prod". Par exemple, si vous optez pour l'option B et utilisez le tenant parent pour la facturation, et avez un sous-tenant "prod", vous devrez ajouter le tenant, basculer sur le nouveau tenant, et ajouter vos utilisateurs administrateurs et modérateurs pour le sous-tenant. 

Enfin, pour clarifier, la page de modération des commentaires sera vide avec l'option B pour le tenant parent.