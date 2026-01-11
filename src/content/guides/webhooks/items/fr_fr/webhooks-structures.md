La seule structure envoyée via les webhooks est l'objet WebhookComment, décrit en TypeScript ci-dessous.

#### Structure de l'objet WebhookComment

##### The "Create" Event Structure
Le corps de la requête de l'événement "create" est un objet WebhookComment.

##### The "Update" Event Structure
Le corps de la requête de l'événement "update" est un objet WebhookComment.

##### The "Delete" Event Structure
Le corps de la requête de l'événement "delete" est un objet WebhookComment.

    Changement à partir du 14 novembre 2023
    Auparavant, le corps de la requête de l'événement "delete" ne contenait que l'identifiant du commentaire. Il contient maintenant le commentaire complet au moment de la suppression.


[inline-code-attrs-start title = 'Objet WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'identifiant du commentaire. **/
    id: string
    /** L'identifiant ou l'URL qui identifie le fil de commentaires. Normalisé. **/
    urlId: string
    /** L'URL qui pointe vers l'endroit où le commentaire a été laissé. **/
    url?: string
    /** L'identifiant de l'utilisateur qui a laissé le commentaire. Si SSO, préfixé par le tenant id. **/
    userId?: string
    /** L'email de l'utilisateur qui a laissé le commentaire. **/
    commenterEmail?: string
    /** Le nom de l'utilisateur affiché dans le widget de commentaire. Avec SSO, peut être displayName. **/
    commenterName: string
    /** Texte brut du commentaire. **/
    comment: string
    /** Texte du commentaire après parsing. **/
    commentHTML: string
    /** Identifiant externe du commentaire. **/
    externalId?: string
    /** L'identifiant du commentaire parent. **/
    parentId?: string | null
    /** La date UTC à laquelle le commentaire a été laissé. **/
    date: UTC_ISO_DateString
    /** Karma combiné (positifs - négatifs) des votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Vrai si l'utilisateur était connecté lorsqu'il a commenté, si son commentaire a été vérifié, ou s'il a vérifié sa session lorsque le commentaire a été laissé. **/
    verified: boolean
    /** Date à laquelle le commentaire a été vérifié. **/
    verifiedDate?: number
    /** Si un modérateur a marqué le commentaire comme examiné. **/
    reviewed: boolean
    /** L'emplacement, ou l'encodage base64, de l'avatar. Ne sera en base64 que si cette valeur a été fournie avec le SSO. **/
    avatarSrc?: string
    /** Le commentaire a-t-il été marqué manuellement ou automatiquement comme spam ? **/
    isSpam: boolean
    /** Le commentaire a-t-il été automatiquement marqué comme spam ? **/
    aiDeterminedSpam: boolean
    /** Y a-t-il des images dans le commentaire ? **/
    hasImages: boolean
    /** Le numéro de page sur lequel se trouve le commentaire pour le tri "Most Relevant". **/
    pageNumber: number
    /** Le numéro de page sur lequel se trouve le commentaire pour le tri "Oldest First". **/
    pageNumberOF: number
    /** Le numéro de page sur lequel se trouve le commentaire pour le tri "Newest First". **/
    pageNumberNF: number
    /** Le commentaire a-t-il été approuvé automatiquement ou manuellement ? **/
    approved: boolean
    /** Le code de locale (format : en_us) de l'utilisateur au moment de l'écriture du commentaire. **/
    locale: string
    /** Les @mentions écrites dans le commentaire qui ont été correctement analysées. **/
    mentions?: CommentUserMention[]
    /** Le domaine d'où provient le commentaire. **/
    domain?: string
    /** La liste optionnelle des identifiants de groupes de modération associés à ce commentaire. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Lorsque des utilisateurs sont tagués dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet de cette liste a la structure suivante.

[inline-code-attrs-start title = 'Objet des mentions Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'identifiant de l'utilisateur. Pour les utilisateurs SSO, celui-ci sera préfixé par votre tenant id. **/
    id: string
    /** Le texte final de la mention @, incluant le symbole @. **/
    tag: string
    /** Le texte original de la mention @, incluant le symbole @. **/
    rawTag: string
    /** Quel type d'utilisateur a été mentionné. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur se désinscrit des notifications, ceci sera quand même défini sur true. **/
    sent: boolean
}
[inline-code-end]

#### Méthodes HTTP utilisées

**Create et Update utilisent tous deux HTTP PUT et non POST !**

Étant donné que toutes nos requêtes contiennent un ID, répéter la même requête Create ou Update ne devrait pas créer de nouveaux objets de votre côté.

Cela signifie que ces appels sont idempotents et doivent être des événements PUT conformément à la spécification HTTP.

---