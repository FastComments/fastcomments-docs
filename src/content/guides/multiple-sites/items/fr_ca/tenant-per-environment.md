Il est courant d'avoir un sub tenant par environnement de test ou de développement avec FastComments. Chaque tenant aurait sa propre configuration, ses propres données et ses propres clés API. La configuration, les données et les utilisateurs ne peuvent pas être partagés entre tenants.
Tout est isolé. Cependant, les super administrateurs du tenant parent peuvent se faire passer pour des utilisateurs des tenants enfants.

Il existe deux approches :

- Le tenant principal sert à la production, et les sub-tenants sont pour les environnements de test.
- Le tenant principal sert simplement à la facturation, et chaque sub-tenant est destiné à la prod, aux tests, etc.

La première est généralement plus simple à comprendre pour les utilisateurs, mais cela peut dépendre de votre organisation.

Les tenants peuvent être créés [ici](https://eu.fastcomments.com/auth/my-account/tenants) si vous avez le package. C'est aussi là que les super administrateurs peuvent
se faire passer pour des utilisateurs. Les tenants peuvent également être créés via l'API pour des configurations plus personnalisées/automatisées.

Quelle que soit l'approche choisie, vous devrez ajouter les modérateurs et les utilisateurs qui veulent voir les données de production dans le tenant "prod". Par exemple, si vous
choisissez l'option B et que vous utilisez le tenant parent pour la facturation, et que vous avez un sub-tenant pour la "prod", vous devrez ajouter le tenant, basculer vers le nouveau tenant, et ajouter vos
utilisateurs administrateurs et modérateurs pour le sub-tenant. 

Enfin, pour clarifier, la page Moderate Comments sera vide avec l'option B pour le tenant parent.