The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### The WebhookComment Object Structure

##### The "Create" Event Structure
The "create" event request body is a WebhookComment object.

##### The "Update" Event Structure
The "update" event request body is a WebhookComment object.

##### The "Delete" Event Structure
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'El objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** El id del comentario. **/
    id: string
    /** El id o URL que identifica el hilo del comentario. Normalizado. **/
    urlId: string
    /** La URL que apunta a donde se dejó el comentario. **/
    url: string | null
    /** El id de usuario que dejó el comentario. Si SSO, con prefijo del id del inquilino. **/
    userId: string | null
    /** El correo electrónico del usuario que dejó el comentario. **/
    commenterEmail: string | null
    /** El nombre del usuario que se muestra en el widget de comentarios. Con SSO, puede ser displayName. **/
    commenterName: string
    /** Texto bruto del comentario. **/
    comment: string
    /** Texto del comentario después del parseo. **/
    commentHTML: string
    /** Id externo del comentario. **/
    externalId: string | null
    /** El id del comentario padre. **/
    parentId: string | null
    /** La fecha UTC cuando se dejó el comentario. **/
    date: UTC_ISO_DateString
    /** Karma combinado (up - down) de los votos. **/
    votes: number
    votesUp: number
    votesDown: number
    /** true si el usuario estaba conectado cuando comentó, o verificó el comentario, o si verificó su sesión cuando se dejó el comentario. **/
    verified: boolean
    /** La fecha UTC cuando el comentario fue verificado. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Si un moderador marcó el comentario como revisado. **/
    reviewed: boolean
    /** La ubicación, o codificación base64, del avatar. Solo será base64 si ese fue el valor pasado con SSO. **/
    avatarSrc: string | null
    /** ¿Fue el comentario marcado manual o automáticamente como spam? **/
    isSpam: boolean
    /** ¿Fue el comentario marcado automáticamente como spam? **/
    aiDeterminedSpam: boolean
    /** ¿Hay imágenes en el comentario? **/
    hasImages: boolean
    /** El número de página en la que se encuentra el comentario para la dirección de ordenación "Most Relevant". **/
    pageNumber: number | null
    /** El número de página en la que se encuentra el comentario para la dirección de ordenación "Oldest First". **/
    pageNumberOF: number | null
    /** El número de página en la que se encuentra el comentario para la dirección de ordenación "Newest First". **/
    pageNumberNF: number | null
    /** ¿Fue el comentario aprobado automáticamente o manualmente? **/
    approved: boolean
    /** El código de locale (formato: en_us) del usuario cuando se escribió el comentario. **/
    locale: string | null
    /** Las @mentions escritas en el comentario que fueron parseadas exitosamente. Vacío cuando no hay ninguna. **/
    mentions: CommentUserMention[]
    /** El dominio del que proviene el comentario. **/
    domain: string | null
    /** Los ids de los grupos de moderación asociados a este comentario. Vacío cuando no hay ninguno. **/
    moderationGroupIds: string[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'El objeto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** El id de usuario. Para usuarios SSO, tendrá el id del inquilino como prefijo. **/
    id: string
    /** El texto final de la etiqueta @mention, incluyendo el símbolo @. **/
    tag: string
    /** El texto original de la etiqueta @mention, incluyendo el símbolo @. **/
    rawTag: string
    /** Qué tipo de usuario fue etiquetado. user = cuenta de FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si el usuario opta por no recibir notificaciones, esto seguirá siendo true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

You can configure the HTTP method for each webhook event type in the admin panel:

- **Evento Create**: POST o PUT (por defecto: PUT)
- **Evento Update**: POST o PUT (por defecto: PUT)
- **Evento Delete**: DELETE, POST o PUT (por defecto: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### Request Headers

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Tu secreto de API |
| `X-FastComments-Timestamp` | Marca de tiempo Unix (segundos) cuando la solicitud fue firmada |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.