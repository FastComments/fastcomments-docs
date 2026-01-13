FastComments fonctionne avec des sites réservés aux membres en utilisant ce qu'on appelle le SSO, ou authentification unique (single sign-on). Vos membres se connectent à votre site WordPress, mais
n'ont pas à se soucier de créer un compte chez FastComments, ni de se connecter avec les réseaux sociaux, pour commenter. Si vos membres (y compris les administrateurs) sont connectés à
votre site WordPress, ils pourront commenter. Vos administrateurs et modérateurs pourront également modérer les commentaires directement depuis vos articles de blog WordPress.

<sup>(Optionnel)</sup> N'oubliez pas d'ajouter aussi vos administrateurs à [Utilisateurs & administrateurs](https://fastcomments.com/auth/my-account/users) et vos modérateurs à [Modérateurs de commentaires](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
pour améliorer leur expérience et activer le suivi des statistiques pour les modérateurs.

SSO peut être activé en allant au tableau de bord du plugin et en cliquant sur "SSO Settings".

Vous devrez d'abord activer la fonctionnalité "Anyone can Register" de votre site.

Toutes les informations utilisateur sont transférées de façon transparente et sécurisée à FastComments chaque fois qu'un utilisateur consulte un fil de commentaires via [HMAC](https://en.wikipedia.org/wiki/HMAC).

Il n'est pas nécessaire d'exécuter une synchronisation initiale ou continue pour copier vos membres sur les serveurs de FastComments. Cela se fait automatiquement lorsqu'ils consultent des fils de commentaires en ouvrant un article de blog.

## Noms et avatars

Le plugin mettra automatiquement à jour le nom affiché de l'utilisateur et son avatar sur tous ses commentaires dans FastComments lorsqu'il consulte
un fil de commentaires. Les avatars sont pris en charge via Gravatar ou tout plugin de gestion d'avatars dans WordPress car le plugin appellera `get_avatar_url`.