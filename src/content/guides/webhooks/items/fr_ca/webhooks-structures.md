La seule structure envoyée via les webhooks est l'objet WebhookComment, décrit en TypeScript ci-dessous.

#### Structure de l'objet WebhookComment

##### Structure de l'événement "create"
Le corps de la requête de l'événement "create" est un objet WebhookComment.

##### Structure de l'événement "update"
Le corps de la requête de l'événement "update" est un objet WebhookComment.

##### Structure de l'événement "delete"
Le corps de la requête de l'événement "delete" est un objet WebhookComment.

    Modification en date du 14 novembre 2023
    Auparavant, le corps de la requête de l'événement "delete" ne contenait que l'id du commentaire. Il contient maintenant le commentaire complet au moment de la suppression.


[inline-code-attrs-start title = 'L&apos;objet WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'id du commentaire. **/
    id: string
    /** L'id ou l'URL qui identifie le fil de commentaires. Normalisé. **/
    urlId: string
    /** L'URL qui pointe vers l'endroit où le commentaire a été laissé. **/
    url?: string
    /** L'id de l'utilisateur qui a laissé le commentaire. Si SSO, préfixé par l'id du tenant. **/
    userId?: string
    /** L'email de l'utilisateur qui a laissé le commentaire. **/
    commenterEmail?: string
    /** Le nom de l'utilisateur affiché dans le widget de commentaire. Avec SSO, peut être displayName. **/
    commenterName: string
    /** Texte brut du commentaire. **/
    comment: string
    /** Texte du commentaire après parsing. **/
    commentHTML: string
    /** Id externe du commentaire. **/
    externalId?: string
    /** L'id du commentaire parent. **/
    parentId?: string | null
    /** La date UTC à laquelle le commentaire a été laissé. **/
    date: UTC_ISO_DateString
    /** Karma combiné (up - down) des votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Vrai si l'utilisateur était connecté lorsqu'il a commenté, ou si son commentaire a été vérifié, ou s'il a vérifié sa session lorsque le commentaire a été laissé. **/
    verified: boolean
    /** Date à laquelle le commentaire a été vérifié. **/
    verifiedDate?: number
    /** Si un modérateur a marqué le commentaire comme examiné. **/
    reviewed: boolean
    /** L'emplacement, ou l'encodage base64, de l'avatar. Ne sera en base64 que si cette valeur a été fournie avec le SSO. **/
    avatarSrc?: string
    /** Le commentaire a-t-il été marqué comme spam manuellement ou automatiquement ? **/
    isSpam: boolean
    /** Le commentaire a-t-il été marqué automatiquement comme spam ? **/
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
    /** Le code de locale (format: en_us) de l'utilisateur au moment où le commentaire a été écrit. **/
    locale: string
    /** Les @mentions présentes dans le commentaire qui ont été correctement analysées. **/
    mentions?: CommentUserMention[]
    /** Le domaine d'origine du commentaire. **/
    domain?: string
    /** La liste optionnelle des ids de groupes de modération associés à ce commentaire. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'L&apos;objet Mentions du Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id de l'utilisateur. Pour les utilisateurs SSO, votre id de tenant sera préfixé. **/
    id: string
    /** Le texte final de la balise @mention, incluant le symbole @. **/
    tag: string
    /** Le texte original de la balise @mention, incluant le symbole @. **/
    rawTag: string
    /** Le type d'utilisateur tagué. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur refuse les notifications, ceci restera tout de même à true. **/
    sent: boolean
}
[inline-code-end]

#### Méthodes HTTP

Vous pouvez configurer la méthode HTTP pour chaque type d'événement webhook dans le panneau d'administration :

- **Événement "create"**: POST ou PUT (par défaut : PUT)
- **Événement "update"**: POST ou PUT (par défaut : PUT)
- **Événement "delete"**: DELETE, POST ou PUT (par défaut : DELETE)

Puisque toutes les requêtes contiennent un ID, les opérations Create et Update sont idempotentes par défaut (PUT). Répéter la même requête Create ou Update ne devrait pas créer d'objets en double de votre côté.

#### En-têtes de requête

Chaque requête webhook inclut les en-têtes suivants :

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Votre secret d'API |
| `X-FastComments-Timestamp` | Timestamp Unix (secondes) lorsque la requête a été signée |
| `X-FastComments-Signature` | Signature HMAC-SHA256 (`sha256=<hex>`) |

Voir [Sécurité et jetons API](/guide-webhooks.html#webhooks-api-tokens) pour des informations sur la vérification de la signature HMAC.