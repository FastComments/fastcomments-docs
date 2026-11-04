La única estructura enviada vía webhooks es el objeto WebhookComment, descrito en TypeScript a continuación.

#### La estructura del objeto WebhookComment

##### La estructura del evento "Create"
El cuerpo de la solicitud del evento "create" es un objeto WebhookComment.

##### La estructura del evento "Update"
El cuerpo de la solicitud del evento "update" es un objeto WebhookComment.

##### La estructura del evento "Delete"
El cuerpo de la solicitud del evento "delete" es un objeto WebhookComment.

    Cambio a partir del 14 de noviembre de 2023
    Anteriormente, el cuerpo de la solicitud del evento "delete" solo contenía el id del comentario. Ahora contiene el comentario completo en el momento de la eliminación.

Cada clave está siempre presente en el cuerpo. Cuando el comentario no tiene valor para un campo, el cuerpo lleva `null`
(o `false` para booleanos y `[]` para listas), por lo que la forma de una entrega nunca varía de un comentario a otro.

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

Cuando se etiqueta a usuarios en un comentario, la información se almacena en una lista llamada `mentions`. Cada objeto de esa lista
tiene la siguiente estructura.

[inline-code-attrs-start title = 'El objeto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
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

Cada entrada de la lista `badges` tiene la siguiente estructura. Consulta la [guía de Badges](/guide-badges.html) para conocer los tipos de badges.

[inline-code-attrs-start title = 'El objeto Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
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

Puedes configurar el método HTTP para cada tipo de evento de webhook en el panel de administración:

- **Evento Create**: POST o PUT (predeterminado: PUT)
- **Evento Update**: POST o PUT (predeterminado: PUT)
- **Evento Delete**: DELETE, POST o PUT (predeterminado: DELETE)

Dado que todas las solicitudes contienen un ID, las operaciones Create y Update son idempotentes por defecto (PUT). Repetir la misma solicitud Create o Update no debería crear objetos duplicados en tu lado.

#### Cabeceras de la solicitud

Cada solicitud de webhook incluye las siguientes cabeceras:

| Cabecera | Descripción |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Tu API Secret |
| `X-FastComments-Timestamp` | Marca de tiempo Unix (segundos) cuando se firmó la solicitud |
| `X-FastComments-Signature` | Firma HMAC-SHA256 (`sha256=<hex>`) |

Consulta [Seguridad y tokens de API](/guide-webhooks.html#webhooks-api-tokens) para obtener información sobre cómo verificar la firma HMAC.