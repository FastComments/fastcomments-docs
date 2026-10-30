The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### La estructura del objeto WebhookComment

##### La estructura del evento "Create"
The "create" event request body is a WebhookComment object.

##### La estructura del evento "Update"
The "update" event request body is a WebhookComment object.

##### La estructura del evento "Delete"
The "delete" event request body is a WebhookComment object.

Change as of Nov 14th 2023  
Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Cada clave siempre está presente en el cuerpo. Cuando el comentario no tiene valor para un campo, el cuerpo lleva `null` (or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'El objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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

Cuando los usuarios son etiquetados en un comentario, la información se almacena en una lista llamada `mentions`. Cada objeto en esa lista tiene la siguiente estructura.

[inline-code-attrs-start title = 'El objeto de menciones Webhook'; type = 'typescript'; inline-code-attrs-end]
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

Cada entrada en la lista `badges` tiene la siguiente estructura. Consulte la [Guía de insignias](/guide-badges.html) para los tipos de insignias.

[inline-code-attrs-start title = 'El objeto de insignia Webhook'; type = 'typescript'; inline-code-attrs-end]
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

#### Métodos HTTP

Puede configurar el método HTTP para cada tipo de evento webhook en el panel de administración:

- **Evento Create**: POST o PUT (predeterminado: PUT)
- **Evento Update**: POST o PUT (predeterminado: PUT)
- **Evento Delete**: DELETE, POST o PUT (predeterminado: DELETE)

Dado que todas las solicitudes contienen un ID, las operaciones Create y Update son idempotentes por defecto (PUT). Repetir la misma solicitud Create o Update no debería crear objetos duplicados en su lado.

#### Encabezados de solicitud

Cada solicitud webhook incluye los siguientes encabezados:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Su secreto de API |
| `X-FastComments-Timestamp` | Marca de tiempo Unix (segundos) cuando la solicitud fue firmada |
| `X-FastComments-Signature` | Firma HMAC-SHA256 (`sha256=<hex>`) |

Consulte [Seguridad y tokens de API](/guide-webhooks.html#webhooks-api-tokens) para obtener información sobre la verificación de la firma HMAC.