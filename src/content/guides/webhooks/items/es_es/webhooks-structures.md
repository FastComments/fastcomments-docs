La única estructura enviada vía webhooks es el objeto WebhookComment, descrito en TypeScript a continuación.

#### Estructura del objeto WebhookComment

##### Estructura del evento "Create"
El cuerpo de la petición del evento "create" es un objeto WebhookComment.

##### Estructura del evento "Update"
El cuerpo de la petición del evento "update" es un objeto WebhookComment.

##### Estructura del evento "Delete"
El cuerpo de la petición del evento "delete" es un objeto WebhookComment.

    Cambio a partir del 14 de noviembre de 2023
    Anteriormente el cuerpo de la petición del evento "delete" solo contenía el id del comentario. Ahora contiene el comentario completo en el momento de la eliminación.


[inline-code-attrs-start title = 'El objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** El id del comentario. **/
    id: string
    /** El id o URL que identifica el hilo de comentarios. Normalizado. **/
    urlId: string
    /** La URL que apunta al lugar donde se dejó el comentario. **/
    url?: string
    /** El id del usuario que dejó el comentario. Si es SSO, con el id del tenant como prefijo. **/
    userId?: string
    /** El correo electrónico del usuario que dejó el comentario. **/
    commenterEmail?: string
    /** El nombre del usuario que se muestra en el widget de comentarios. Con SSO, puede ser displayName. **/
    commenterName: string
    /** Texto sin procesar del comentario. **/
    comment: string
    /** Texto del comentario tras el parseo. **/
    commentHTML: string
    /** Id externo del comentario. **/
    externalId?: string
    /** El id del comentario padre. **/
    parentId?: string | null
    /** La fecha UTC cuando se dejó el comentario. **/
    date: UTC_ISO_DateString
    /** Karma combinado (positivos - negativos) de los votos. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True si el usuario estaba logueado cuando comentó, o si verificó el comentario, o si verificó su sesión cuando se dejó el comentario. **/
    verified: boolean
    /** Fecha cuando se verificó el comentario. **/
    verifiedDate?: number
    /** Si un moderador marcó el comentario como revisado. **/
    reviewed: boolean
    /** La ubicación, o codificación base64, del avatar. Será base64 sólo si ese fue el valor pasado con SSO. **/
    avatarSrc?: string
    /** ¿El comentario fue marcado manual o automáticamente como spam? **/
    isSpam: boolean
    /** ¿El comentario fue marcado automáticamente como spam? **/
    aiDeterminedSpam: boolean
    /** ¿Hay imágenes en el comentario? **/
    hasImages: boolean
    /** El número de página en el que está el comentario para la dirección de ordenación "Most Relevant". **/
    pageNumber: number
    /** El número de página en el que está el comentario para la dirección de ordenación "Oldest First". **/
    pageNumberOF: number
    /** El número de página en el que está el comentario para la dirección de ordenación "Newest First". **/
    pageNumberNF: number
    /** ¿El comentario fue aprobado automáticamente o manualmente? **/
    approved: boolean
    /** El código de localización (formato: en_us) del usuario cuando se escribió el comentario. **/
    locale: string
    /** Las @menciones escritas en el comentario que fueron parseadas correctamente. **/
    mentions?: CommentUserMention[]
    /** El dominio del que proviene el comentario. **/
    domain?: string
    /** La lista opcional de ids de grupo de moderación asociada con este comentario. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Cuando los usuarios son etiquetados en un comentario, la información se almacena en una lista llamada `mentions`. Cada objeto en esa lista
tiene la siguiente estructura.

[inline-code-attrs-start title = 'El objeto de menciones del Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** El id del usuario. Para usuarios SSO, tendrá el id de tu tenant como prefijo. **/
    id: string
    /** El texto final de la etiqueta @mention, incluyendo el símbolo @. **/
    tag: string
    /** El texto original de la etiqueta @mention, incluyendo el símbolo @. **/
    rawTag: string
    /** Qué tipo de usuario fue etiquetado. user = cuenta de FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si el usuario se da de baja de las notificaciones, esto seguirá establecido en true. **/
    sent: boolean
}
[inline-code-end]

#### Métodos HTTP

Puedes configurar el método HTTP para cada tipo de evento de webhook en el panel de administración:

- **Evento Create**: POST o PUT (predeterminado: PUT)
- **Evento Update**: POST o PUT (predeterminado: PUT)
- **Evento Delete**: DELETE, POST o PUT (predeterminado: DELETE)

Dado que todas las solicitudes contienen un ID, las operaciones Create y Update son idempotentes por defecto (PUT). Repetir la misma petición Create o Update no debería crear objetos duplicados en tu lado.

#### Encabezados de la petición

Cada petición de webhook incluye los siguientes encabezados:

| Encabezado | Descripción |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Tu secreto de API |
| `X-FastComments-Timestamp` | Marca de tiempo Unix (segundos) cuando la petición fue firmada |
| `X-FastComments-Signature` | Firma HMAC-SHA256 (`sha256=<hex>`) |

Consulta [Seguridad y tokens de API](/guides/webhooks/webhooks-api-tokens) para obtener información sobre cómo verificar la firma HMAC.

---