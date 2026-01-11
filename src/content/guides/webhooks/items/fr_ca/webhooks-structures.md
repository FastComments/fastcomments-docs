La seule structure envoyée via les webhooks est l'objet WebhookComment, décrit en TypeScript ci‑dessous.

#### La structure de l'événement "Create"
Le corps de la requête de l'événement "create" est un objet WebhookComment.

#### La structure de l'événement "Update"
Le corps de la requête de l'événement "update" est un objet WebhookComment.

#### La structure de l'événement "Delete"
Le corps de la requête de l'événement "delete" est un objet WebhookComment.

    Changement au 14 novembre 2023
    Auparavant, le corps de la requête de l'événement "delete" ne contenait que l'identifiant du commentaire. Il contient maintenant le commentaire complet au moment de la suppression.


[inline-code-attrs-start title = 'Objet WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'id du commentaire. **/
    id: string
    /** L'id ou l'URL qui identifie le fil de commentaires. Normalisé. **/
    urlId: string
    /** L'URL pointant vers l'endroit où le commentaire a été laissé. **/
    url?: string
    /** L'id de l'utilisateur qui a laissé le commentaire. Si SSO, préfixé avec l'id du locataire. **/
    userId?: string
    /** L'email de l'utilisateur ayant laissé le commentaire. **/
    commenterEmail?: string
    /** Le nom de l'utilisateur qui apparaît dans le widget de commentaires. Avec SSO, peut être displayName. **/
    commenterName: string
    /** Texte brut du commentaire. **/
    comment: string
    /** Texte du commentaire après analyse. **/
    commentHTML: string
    /** Identifiant externe du commentaire. **/
    externalId?: string
    /** L'id du commentaire parent. **/
    parentId?: string | null
    /** La date UTC à laquelle le commentaire a été laissé. **/
    date: UTC_ISO_DateString
    /** Karma combiné (pour - contre) des votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Vrai si l'utilisateur était connecté lorsqu'il a commenté, ou si son commentaire a été vérifié, ou s'il a vérifié sa session au moment du commentaire. **/
    verified: boolean
    /** Date à laquelle le commentaire a été vérifié. **/
    verifiedDate?: number
    /** Si un modérateur a marqué le commentaire comme révisé. **/
    reviewed: boolean
    /** L'emplacement, ou l'encodage base64, de l'avatar. Sera en base64 seulement si cette valeur a été fournie avec le SSO. **/
    avatarSrc?: string
    /** Le commentaire a-t-il été marqué comme spam manuellement ou automatiquement ? **/
    isSpam: boolean
    /** Le commentaire a-t-il été automatiquement marqué comme spam ? **/
    aiDeterminedSpam: boolean
    /** Y a-t-il des images dans le commentaire ? **/
    hasImages: boolean
    /** Le numéro de page sur lequel se trouve le commentaire pour l'ordre de tri "Most Relevant". **/
    pageNumber: number
    /** Le numéro de page sur lequel se trouve le commentaire pour l'ordre de tri "Oldest First". **/
    pageNumberOF: number
    /** Le numéro de page sur lequel se trouve le commentaire pour l'ordre de tri "Newest First". **/
    pageNumberNF: number
    /** Le commentaire a-t-il été approuvé automatiquement ou manuellement ? **/
    approved: boolean
    /** Le code de locale (format : en_us) de l'utilisateur au moment où le commentaire a été écrit. **/
    locale: string
    /** Les @mentions écrites dans le commentaire qui ont été correctement analysées. **/
    mentions?: CommentUserMention[]
    /** Le domaine d'où provient le commentaire. **/
    domain?: string
    /** La liste optionnelle des ids de groupe de modération associés à ce commentaire. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Lorsque des utilisateurs sont tagués dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet de cette liste a la structure suivante.

[inline-code-attrs-start title = 'Objet des mentions du webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id de l'utilisateur. Pour les utilisateurs SSO, celui-ci comportera votre id de locataire en préfixe. **/
    id: string
    /** Le texte final de la mention @, incluant le symbole @. **/
    tag: string
    /** Le texte original de la mention @, incluant le symbole @. **/
    rawTag: string
    /** Quel type d'utilisateur a été mentionné. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur se désabonne des notifications, ceci sera tout de même défini sur true. **/
    sent: boolean
}
[inline-code-end]

#### Méthodes HTTP utilisées

**Create et Update utilisent tous deux HTTP PUT et non POST !**

Puisque toutes nos requêtes contiennent un ID, répéter la même requête Create ou Update ne devrait pas créer de nouveaux objets de votre côté.

Cela signifie que ces appels sont idempotents et devraient être des événements PUT conformément à la spécification HTTP.