Un objet `Moderator` représente la configuration d'un modérateur.

Il existe trois types de modérateurs :

1. Les utilisateurs administrateurs qui ont le drapeau `isCommentModeratorAdmin`.
2. Les utilisateurs SSO avec le drapeau `isCommentModeratorAdmin`.
3. Les commentateurs réguliers, ou utilisateurs FastComments.com, qui sont invités en tant que modérateurs.

La structure `Moderator` est utilisée pour représenter l'état de modération du cas d'utilisation `3`.

Si vous souhaitez inviter un utilisateur à être modérateur, via l'API, utilisez l'API `Moderator` en créant un `Moderator` et en l'`invitant`.

Si l'utilisateur n'a pas de compte FastComments.com, l'email d'invitation l'aidera à se configurer. S'il a déjà un compte, il se verra
accorder l'accès de modération à votre locataire et le `userId` de l'objet `Moderator` sera mis à jour pour pointer vers son utilisateur. Vous n'aurez pas d'accès API
à son utilisateur, car dans ce cas il lui appartient et est géré par FastComments.com.

Si vous avez besoin d'une gestion complète du compte de l'utilisateur, nous recommandons soit d'utiliser SSO, soit de l'ajouter en tant qu'[Utilisateur Locataire](https://fastcomments.com/auth/my-account/users) et
ensuite d'ajouter un objet `Moderator` pour suivre ses statistiques.

La structure `Moderator` peut être utilisée comme mécanisme de suivi des statistiques pour les cas d'utilisation `1` et `2`. Après avoir créé l'utilisateur, ajoutez un objet `Moderator`
avec son `userId` défini et ses statistiques seront suivies sur la [Page des Modérateurs de Commentaires](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

La structure de l'objet `Moderator` est la suivante :

[inline-code-attrs-start title = 'Structure de Modérateur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]
