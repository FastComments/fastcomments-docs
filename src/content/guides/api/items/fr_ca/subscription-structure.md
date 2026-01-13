Un objet `Subscription` représente un abonnement pour un utilisateur.

Les objets `Subscription` sont créés lorsqu'un utilisateur clique sur la cloche de notification dans le widget de commentaires et clique sur "S'abonner à cette page".

Les abonnements peuvent également être créés via l'API.

Avoir un objet `Subscription` provoque la génération d'objets `Notification`, et l'envoi d'emails, lorsque de nouveaux commentaires sont laissés à la racine de la page associée
à laquelle le `Subscription` est destiné. L'envoi d'emails dépend du type d'utilisateur. Pour les utilisateurs réguliers, cela dépend de `optedInNotifications`. Pour les utilisateurs SSO, cela dépend de `optedInSubscriptionNotifications`. Notez que certaines applications peuvent ne pas avoir le concept de page accessible sur le web, auquel cas définissez simplement `urlId` sur
l'id de l'élément auquel vous vous abonnez (même valeur pour `urlId` que vous passeriez au widget de commentaires).

La structure de l'objet `Subscription` est la suivante :

[inline-code-attrs-start title = 'Structure de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]
