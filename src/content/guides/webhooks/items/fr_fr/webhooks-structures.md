La seule structure envoyée via les webhooks est l'objet **WebhookComment**, décrit en TypeScript ci‑dessous.

#### Structure de l'objet WebhookComment

##### Structure de l'événement "Create"
Le corps de la requête de l'événement **"create"** est un objet WebhookComment.

##### Structure de l'événement "Update"
Le corps de la requête de l'événement **"update"** est un objet WebhookComment.

##### Structure de l'événement "Delete"
Le corps de la requête de l'événement **"delete"** est un objet WebhookComment.

    Modification à partir du 14 novembre 2023
    Auparavant, le corps de la requête de l'événement "delete" ne contenait que l'identifiant du commentaire. Il contient maintenant le commentaire complet au moment de la suppression.

Chaque clé est toujours présente dans le corps. Lorsque le commentaire n'a pas de valeur pour un champ, le corps contient `null` (ou `false` pour les booléens et `[]` pour les listes), de sorte que la forme d'une livraison ne varie jamais d'un commentaire à l'autre.

[inline-code-attrs-start title = 'L\'objet WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Lorsque des utilisateurs sont mentionnés dans un commentaire, l'information est stockée dans une liste appelée `mentions`. Chaque objet de cette liste possède la structure suivante.

[inline-code-attrs-start title = 'L\'objet Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

Chaque entrée de la liste `badges` possède la structure suivante. Consultez le [guide des badges](/guide-badges.html) pour les types de badges.

[inline-code-attrs-start title = 'L\'objet Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### Méthodes HTTP

Vous pouvez configurer la méthode HTTP pour chaque type d'événement webhook dans le panneau d'administration :

- **Événement de création** : POST ou PUT (par défaut : PUT)
- **Événement de mise à jour** : POST ou PUT (par défaut : PUT)
- **Événement de suppression** : DELETE, POST ou PUT (par défaut : DELETE)

Comme toutes les requêtes contiennent un ID, les opérations de création et de mise à jour sont idempotentes par défaut (PUT). Répéter la même requête de création ou de mise à jour ne doit pas créer d'objets en double de votre côté.

#### En‑têtes de requête

Chaque requête webhook inclut les en‑têtes suivants :

| En‑tête | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Votre secret d'API |
| `X-FastComments-Timestamp` | Horodatage Unix (secondes) au moment où la requête a été signée |
| `X-FastComments-Signature` | Signature HMAC‑SHA256 (`sha256=<hex>`) |

Voir [Sécurité & jetons d'API](/guide-webhooks.html#webhooks-api-tokens) pour des informations sur la vérification de la signature HMAC.