La única estructura enviada vía webhooks es el objeto WebhookComment, descrito en TypeScript a continuación.

#### La estructura del objeto WebhookComment

##### Estructura del evento "create"
El cuerpo de la solicitud del evento "create" es un objeto WebhookComment.

##### Estructura del evento "update"
El cuerpo de la solicitud del evento "update" es un objeto WebhookComment.

##### Estructura del evento "delete"
El cuerpo de la solicitud del evento "delete" es un objeto WebhookComment.

    Cambio a partir del 14 de nov de 2023
    Anteriormente el cuerpo de la solicitud del evento "delete" solo contenía el id del comentario. Ahora contiene el comentario completo en el momento de la eliminación.


[inline-code-attrs-start title = 'El objeto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** El id del comentario. **/
    id: string
    /** El id o URL que identifica el hilo de comentarios. Normalizado. **/
    urlId: string
    /** La URL que apunta a donde se dejó el comentario. **/
    url?: string
    /** El id de usuario que dejó el comentario. Si es SSO, está prefijado con el id del tenant. **/
    userId?: string
    /** El correo electrónico del usuario que dejó el comentario. **/
    commenterEmail?: string
    /** El nombre del usuario que aparece en el widget de comentarios. Con SSO, puede ser displayName. **/
    commenterName: string
    /** Texto bruto del comentario. **/
    comment: string
    /** Texto del comentario después del parseo. **/
    commentHTML: string
    /** Id externo del comentario. **/
    externalId?: string
    /** El id del comentario padre. **/
    parentId?: string | null
    /** La fecha UTC en que se dejó el comentario. **/
    date: UTC_ISO_DateString
    /** Karma combinado (votos a favor - votos en contra). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Verdadero si el usuario había iniciado sesión cuando comentó, o si verificaron el comentario, o si verificaron su sesión cuando se dejó el comentario. **/
    verified: boolean
    /** Fecha cuando el comentario fue verificado. **/
    verifiedDate?: number
    /** Si un moderador marcó el comentario como revisado. **/
    reviewed: boolean
    /** La ubicación, o codificación base64, del avatar. Solo será base64 si ese fue el valor pasado con SSO. **/
    avatarSrc?: string
    /** ¿El comentario fue marcado como spam manual o automáticamente? **/
    isSpam: boolean
    /** ¿El comentario fue marcado automáticamente como spam? **/
    aiDeterminedSpam: boolean
    /** ¿Hay imágenes en el comentario? **/
    hasImages: boolean
    /** El número de página en el que está el comentario para la dirección de orden "Most Relevant". **/
    pageNumber: number
    /** El número de página en el que está el comentario para la dirección de orden "Oldest First". **/
    pageNumberOF: number
    /** El número de página en el que está el comentario para la dirección de orden "Newest First". **/
    pageNumberNF: number
    /** ¿El comentario fue aprobado automáticamente o manualmente? **/
    approved: boolean
    /** El código de localización (formato: en_us) del usuario cuando se escribió el comentario. **/
    locale: string
    /** Las @menciones escritas en el comentario que fueron analizadas correctamente. **/
    mentions?: CommentUserMention[]
    /** El dominio del que proviene el comentario. **/
    domain?: string
    /** La lista opcional de ids de grupo de moderación asociados a este comentario. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Cuando los usuarios son etiquetados en un comentario, la información se almacena en una lista llamada `mentions`. Cada objeto en esa lista
tiene la siguiente estructura.

[inline-code-attrs-start title = 'El objeto de menciones del webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** El id del usuario. Para usuarios SSO, tendrá prefijado el id de su tenant. **/
    id: string
    /** El texto final de la etiqueta @mention, incluyendo el símbolo @. **/
    tag: string
    /** El texto original de la etiqueta @mention, incluyendo el símbolo @. **/
    rawTag: string
    /** Qué tipo de usuario fue etiquetado. user = cuenta de FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si el usuario opta por no recibir notificaciones, esto aún estará establecido en true. **/
    sent: boolean
}
[inline-code-end]

#### Métodos HTTP usados

**¡Los eventos 'create' y 'update' usan HTTP PUT y no POST!**

Dado que todas nuestras solicitudes contienen un ID, repetir la misma solicitud Create o Update no debería crear nuevos objetos en su lado.

Esto significa que estas llamadas son idempotentes y deberían ser eventos PUT según la especificación HTTP.