La seule structure envoyée via les webhooks est l'objet WebhookComment, décrit en TypeScript ci-dessous.

#### Structure de l'objet WebhookComment

##### Structure de l'événement "create"
Le corps de la requête de l'événement "create" est un objet WebhookComment.

##### Structure de l'événement "update"
Le corps de la requête de l'événement "update" est un objet WebhookComment.

##### Structure de l'événement "delete"
Le corps de la requête de l'événement "delete" est un objet WebhookComment.

    Modification du 14 nov. 2023
    Auparavant, le corps de la requête de l'événement "delete" ne contenait que l'id du commentaire. Il contient maintenant le commentaire complet au moment de la suppression.


[inline-code-attrs-start title = 'L\'objet WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'identifiant du commentaire. **/
    id: string
    /** L'identifiant ou l'URL qui identifie le fil de commentaires. Normalisé. **/
    urlId: string
    /** L'URL pointant vers l'endroit où le commentaire a été laissé. **/
    url?: string
    /** L'ID de l'utilisateur qui a laissé le commentaire. Si SSO, préfixé par l'ID du locataire. **/
    userId?: string
    /** L'email de l'utilisateur ayant laissé le commentaire. **/
    commenterEmail?: string
    /** Le nom de l'utilisateur affiché dans le widget de commentaires. Avec SSO, peut être displayName. **/
    commenterName: string
    /** Texte brut du commentaire. **/
    comment: string
    /** Texte du commentaire après analyse. **/
    commentHTML: string
    /** Identifiant externe du commentaire. **/
    externalId?: string
    /** L'ID du commentaire parent. **/
    parentId?: string | null
    /** La date UTC à laquelle le commentaire a été laissé. **/
    date: UTC_ISO_DateString
    /** Karma combiné (up - down) des votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Vrai si l'utilisateur était connecté lorsqu'il a commenté, si son commentaire a été vérifié, ou s'il a vérifié sa session lorsque le commentaire a été laissé. **/
    verified: boolean
    /** Date à laquelle le commentaire a été vérifié. **/
    verifiedDate?: number
    /** Si un modérateur a marqué le commentaire comme examiné. **/
    reviewed: boolean
    /** L'emplacement ou l'encodage base64 de l'avatar. Sera en base64 uniquement si cette valeur a été fournie avec SSO. **/
    avatarSrc?: string
    /** Le commentaire a-t-il été marqué comme spam manuellement ou automatiquement ? **/
    isSpam: boolean
    /** Le commentaire a-t-il été marqué automatiquement comme spam ? **/
    aiDeterminedSpam: boolean
    /** Y a-t-il des images dans le commentaire ? **/
    hasImages: boolean
    /** Le numéro de page sur lequel se trouve le commentaire pour la direction de tri "Most Relevant". **/
    pageNumber: number
    /** Le numéro de page sur lequel se trouve le commentaire pour la direction de tri "Oldest First". **/
    pageNumberOF: number
    /** Le numéro de page sur lequel se trouve le commentaire pour la direction de tri "Newest First". **/
    pageNumberNF: number
    /** Le commentaire a-t-il été approuvé automatiquement ou manuellement ? **/
    approved: boolean
    /** Le code de locale (format: en_us) de l'utilisateur au moment où le commentaire a été rédigé. **/
    locale: string
    /** Les @mentions écrites dans le commentaire qui ont été correctement analysées. **/
    mentions?: CommentUserMention[]
    /** Le domaine d'où provient le commentaire. **/
    domain?: string
    /** La liste optionnelle des IDs de groupes de modération associés à ce commentaire. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Lorsque des utilisateurs sont tagués dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet de cette liste a la structure suivante.

[inline-code-attrs-start title = 'L\'objet de mentions du webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'ID de l'utilisateur. Pour les utilisateurs SSO, votre ID de locataire sera préfixé. **/
    id: string
    /** Le texte final de la balise @mention, incluant le symbole @. **/
    tag: string
    /** Le texte original de la balise @mention, incluant le symbole @. **/
    rawTag: string
    /** Quel type d'utilisateur a été tagué. user = compte FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si l'utilisateur se désinscrit des notifications, ceci restera quand même défini à true. **/
    sent: boolean
}
[inline-code-end]

#### Méthodes HTTP

Vous pouvez configurer la méthode HTTP pour chaque type d'événement webhook dans le panneau d'administration :

- **Événement « create »** : POST ou PUT (par défaut : PUT)
- **Événement « update »** : POST ou PUT (par défaut : PUT)
- **Événement « delete »** : DELETE, POST ou PUT (par défaut : DELETE)

Puisque toutes les requêtes contiennent un ID, les opérations Create et Update sont idempotentes par défaut (PUT). Répéter la même requête Create ou Update ne devrait pas créer d'objets en double de votre côté.

#### En-têtes de requête

Chaque requête webhook inclut les en-têtes suivants :

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Votre secret d'API |
| `X-FastComments-Timestamp` | Timestamp Unix (secondes) indiquant le moment où la requête a été signée |
| `X-FastComments-Signature` | Signature HMAC-SHA256 (`sha256=<hex>`) |

Voir [Sécurité et jetons d'API](/guides/webhooks/webhooks-api-tokens) pour des informations sur la vérification de la signature HMAC.

---