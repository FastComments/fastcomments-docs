---
FastComments fonctionne avec des sites réservés aux membres en utilisant ce que l'on appelle SSO, ou single-sign-on. Vos membres se connectent à votre site WordPress, mais
n'ont pas à se soucier de créer un compte avec FastComments, ni de se connecter via les réseaux sociaux, pour commenter. Si vos membres (y compris les administrateurs) sont connectés à
votre site WordPress, ils pourront commenter. Vos administrateurs et modérateurs pourront également modérer les commentaires directement depuis vos articles de blog WordPress.

<sup>(Optionnel)</sup> N'oubliez pas d'ajouter également vos administrateurs à [Utilisateurs et administrateurs](https://fastcomments.com/auth/my-account/users) et vos modérateurs à [Modérateurs de commentaires](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
pour améliorer leur expérience et activer le suivi des statistiques pour les modérateurs.

Le SSO peut être activé en allant sur le tableau de bord du plugin et en cliquant sur "Paramètres SSO".

Vous devrez d'abord activer la fonctionnalité « Tout le monde peut s'inscrire » de votre site.

Toutes les informations utilisateur sont transférées de manière transparente et sécurisée vers FastComments chaque fois qu'un utilisateur consulte un fil de commentaires via [HMAC](https://en.wikipedia.org/wiki/HMAC).

Il n'est pas nécessaire d'exécuter une synchronisation initiale ou continue pour copier vos membres sur les serveurs de FastComments. Cela se fait automatiquement lorsqu'ils consultent des fils de commentaires en ouvrant un article de blog.

## Noms et avatars

Le plugin mettra automatiquement à jour le nom affiché de l'utilisateur et son avatar sur tous ses commentaires dans FastComments lorsqu'il consultera n'importe quel fil de commentaires. Les avatars sont pris en charge via Gravatar ou tout plugin de gestion d'avatars dans WordPress puisque le plugin appellera `get_avatar_url`.
---