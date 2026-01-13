La seule structure envoyée via les webhooks est l'objet WebhookComment, décrit en TypeScript ci-dessous.

#### Structure de l'objet WebhookComment

##### Structure de l'événement "Create"
Le corps de la requête pour l'événement "create" est un objet WebhookComment.

##### Structure de l'événement "Update"
Le corps de la requête pour l'événement "update" est un objet WebhookComment.

##### Structure de l'événement "Delete"
Le corps de la requête pour l'événement "delete" est un objet WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Objet WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'identifiant du commentaire. **/
    id: string
    /** L'identifiant ou l'URL qui identifie le fil de commentaires. Normalisé. **/
    urlId: string
    /** L'URL pointant vers l'endroit où le commentaire a été laissé. **/
    url?: string
    /** L'identifiant utilisateur qui a laissé le commentaire. Si SSO, préfixé par l'identifiant du tenant. **/
    userId?: string
    /** L'email de l'utilisateur ayant laissé le commentaire. **/
    commenterEmail?: string
    /** Le nom de l'utilisateur affiché dans le widget de commentaire. Avec SSO, peut être displayName. **/
    commenterName: string
    /** Texte brut du commentaire. **/
    comment: string
    /** Texte du commentaire après analyse. **/
    commentHTML: string
    /** Identifiant externe du commentaire. **/
    externalId?: string
    /** L'identifiant du commentaire parent. **/
    parentId?: string | null
    /** La date UTC à laquelle le commentaire a été laissé. **/
    date: UTC_ISO_DateString
    /** Karma combiné (up - down) des votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Vrai si l'utilisateur était connecté lorsqu'il a commenté, s'il a vérifié le commentaire, ou s'il a vérifié sa session au moment où le commentaire a été laissé. **/
    verified: boolean
    /** Date à laquelle le commentaire a été vérifié. **/
    verifiedDate?: number
    /** Si un modérateur a marqué le commentaire comme revu. **/
    reviewed: boolean
    /** L'emplacement, ou l'encodage base64, de l'avatar. Sera base64 uniquement si cette valeur a été fournie avec le SSO. **/
    avatarSrc?: string
    /** Le commentaire a-t-il été marqué comme spam manuellement ou automatiquement ? **/
    isSpam: boolean
    /** Le commentaire a-t-il été marqué automatiquement comme spam ? **/
    aiDeterminedSpam: boolean
    /** Le commentaire contient-il des images ? **/
    hasImages: boolean
    /** Le numéro de page où se trouve le commentaire pour l'option de tri "Most Relevant". **/
    pageNumber: number
    /** Le numéro de page où se trouve le commentaire pour l'ordre de tri "Oldest First". **/
    pageNumberOF: number
    /** Le numéro de page où se trouve le commentaire pour l'ordre de tri "Newest First". **/
    pageNumberNF: number
    /** Le commentaire a-t-il été approuvé automatiquement ou manuellement ? **/
    approved: boolean
    /** Le code de locale (format : en_us) de l'utilisateur au moment où le commentaire a été rédigé. **/
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

[inline-code-attrs-start title = 'Objet Mentions du Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'identifiant de l'utilisateur. Pour les utilisateurs SSO, cet identifiant sera préfixé par votre identifiant de tenant. **/
    id: string
    /** Le texte final de la balise @mention, incluant le symbole @. **/
    tag: string
    /** Le texte original de la balise @mention, incluant le symbole @. **/
    rawTag: string
    /** Type d'utilisateur mentionné. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur refuse les notifications, cette valeur sera quand même définie sur true. **/
    sent: boolean
}
[inline-code-end]

#### Méthodes HTTP

Vous pouvez configurer la méthode HTTP pour chaque type d'événement webhook dans le panneau d'administration :

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### En-têtes de requête

Chaque requête webhook inclut les en-têtes suivants :

| En-tête | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Votre secret d'API |
| `X-FastComments-Timestamp` | Horodatage Unix (secondes) correspondant au moment de la signature de la requête |
| `X-FastComments-Signature` | Signature HMAC-SHA256 (`sha256=<hex>`) |

Voir [Sécurité & jetons d'API](/guides/webhooks/webhooks-api-tokens) pour obtenir des informations sur la vérification de la signature HMAC.