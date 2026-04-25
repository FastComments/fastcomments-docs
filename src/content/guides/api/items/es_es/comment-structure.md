Un objeto `Comment` representa un comentario dejado por un usuario.

La relación entre comentarios padre e hijo se define mediante `parentId`.

La estructura del objeto Comment es la siguiente:

[inline-code-attrs-start title = 'Estructura de Comment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** SOLO LECTURA: Establecido en true si el motor de spam determinó que el comentario era spam. **/
    aiDeterminedSpam?: boolean
    /** Indica si el comentario está aprobado para mostrarse. Se establece en true al guardar el comentario; de lo contrario quedará oculto. **/
    approved?: boolean
    /** El avatar del usuario. **/
    avatarSrc?: string
    /** Comentarios hijos. No se pueblan en todos los escenarios. Se usan cuando asTree se establece en true vía la API. **/
    children: Comment[]
    /** El comentario sin procesar del autor. **/
    comment: string
    /** SOLO LECTURA: El comentario del autor parseado a HTML. **/
    commentHTML?: string
    /** El correo electrónico del autor. Requerido si la posibilidad de comentar de forma anónima está desactivada. **/
    commenterEmail?: string
    /** El enlace del autor (por ejemplo, su blog). **/
    commenterLink?: string
    /** El nombre del autor del comentario. Siempre requerido. Si no está disponible, poner algo como "Anonymous". **/
    commenterName: string
    /** La fecha en que se dejó el comentario, en epoch UTC. **/
    date: number
    /** La "etiqueta de visualización" del comentario - por ejemplo "Admin", "Moderator", o algo como "VIP User". **/
    displayLabel?: string
    /** El dominio en el que se publicó el comentario. **/
    domain?: string
    /** SOLO LECTURA: El número de veces que el comentario fue marcado. **/
    flagCount?: number
    /** Los #hashtags escritos en el comentario que se analizaron correctamente. También puedes añadir hashtags manualmente para consultas, pero no se mostrarán automáticamente en el texto del comentario. **/
    hashTags?: CommentHashTag[]
    /** SOLO LECTURA: ¿El comentario contiene imágenes? **/
    hasImages?: boolean
    /** SOLO LECTURA: ¿El comentario contiene enlaces? **/
    hasLinks?: boolean
    /** SOLO LECTURA: El id único del comentario. **/
    id: string
    /** ¡Sólo al crear! Esto se hashea para almacenamiento. **/
    ip?: string
    /** SOLO LECTURA: ¿El usuario actual bloqueó al autor de este comentario? **/
    isBlocked?: boolean
    /** SOLO LECTURA: ¿El comentario es de un admin? Se establece automáticamente según userId. **/
    isByAdmin?: boolean
    /** SOLO LECTURA: ¿El comentario es de un moderador? Se establece automáticamente según userId. **/
    isByModerator?: boolean
    /** Se establece en true si el comentario fue eliminado de forma suave (se dejó un marcador debido a alguna otra configuración). **/
    isDeleted?: boolean
    /** Se establece en true si la cuenta del usuario fue eliminada y el comentario tuvo que conservarse. **/
    isDeletedUser?: boolean
    /** SOLO LECTURA: ¿Está marcado por el usuario actualmente conectado (contextUserId)? **/
    isFlagged?: boolean
    /** ¿Está el comentario fijado? **/
    isPinned?: boolean
    /** ¿Está el comentario bloqueado? Cuando es true, nadie (incluidos los moderadores) puede responder, editar ni eliminarlo hasta que se desbloquee. **/
    isLocked?: boolean
    /** ¿Es el comentario spam? **/
    isSpam?: boolean
    /** SOLO LECTURA: ¿El comentario fue votado en contra por el usuario actual (contextUserId)? **/
    isVotedDown?: boolean
    /** SOLO LECTURA: ¿El comentario fue votado a favor por el usuario actual (contextUserId)? **/
    isVotedUp?: boolean
    /** La configuración regional (locale) del comentario. Si no se proporciona, se derivará de la cabecera HTTP accept-language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** SOLO LECTURA: Las @menciones escritas en el comentario que se analizaron correctamente. **/
    mentions?: CommentUserMention[]
    /** Metadatos opcionales asociados con el comentario. **/
    meta?: Record<string, string | number | boolean>
    /** La lista opcional de ids de grupos de moderación asociados con este comentario. **/
    moderationGroupIds?: string[]|null
    /** SOLO LECTURA: El id del objeto de voto que corresponde al voto del usuario actual (contextUserId) sobre este comentario. **/
    myVoteId?: string
    /** Indica si se enviaron notificaciones para este comentario a los comentaristas. Para evitar que se envíen notificaciones en importaciones, establece esto en true. **/
    notificationSentForParent?: boolean
    /** Indica si se enviaron notificaciones para este comentario a los usuarios del tenant. Para evitar que se envíen notificaciones en importaciones, establece esto en true. **/
    notificationSentForParentTenant?: boolean
    /** El título de la página en la que estuvo este comentario. **/
    pageTitle?: string
    /** Si estamos respondiendo a un comentario, este es el ID al que respondemos. **/
    parentId?: string|null
    /** Indica si el comentario está marcado como revisado. **/
    reviewed: boolean
    /** El id del tenant al que pertenece el comentario. **/
    tenantId: string
    /** El usuario que escribió el comentario. Creado automáticamente al guardar un comentario con nombre/email. **/
    userId?: string|null
    /** La URL de la ubicación donde este comentario es visible, como una entrada de blog. **/
    url: string
    /** Una versión "limpia" del urlId que nos pasaste. Al guardar, especificas este campo, pero cuando recuperes el comentario éste será "limpiado" y tu valor original movido a "urlIdRaw". **/
    urlId: string
    /** SOLO LECTURA: El urlId original que nos pasaste. **/
    urlIdRaw?: string
    /** ¿El usuario y este comentario están verificados? **/
    verified: boolean
    /** Número de votos a favor. **/
    votesUp?: number
    /** Número de votos en contra. **/
    votesDown?: number
    /** El "karma" del comentario (= votos a favor - votos en contra). **/
    votes?: number
}
[inline-code-end]

Algunos de estos campos están marcados `READONLY` - estos son devueltos por la API pero no se pueden establecer.

### Estructura del texto del comentario

Los comentarios se escriben en una variante de markdown de FastComments, que es simplemente markdown más etiquetas de estilo `bbcode` tradicionales para imágenes, como `[img]path[/img]`.

El texto se almacena en dos campos. El texto que ingresó el usuario se almacena sin modificar en el campo `comment`. Esto se renderiza y almacena en el campo `commentHTML`.

Las etiquetas HTML permitidas son `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Se recomienda renderizar el HTML, ya que es un subconjunto muy pequeño de HTML; construir un renderizador es bastante sencillo. Por ejemplo, hay múltiples librerías para React Native y Flutter para ayudar con esto

Puedes optar por renderizar el valor no normalizado del campo `comment`. [Un parser de ejemplo está aquí.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

El parser de ejemplo también puede ajustarse para trabajar con HTML y transformar las etiquetas HTML en los elementos esperados para renderizar en tu plataforma. 

### Etiquetado

Cuando los usuarios son etiquetados en un comentario, la información se almacena en una lista llamada `mentions`. Cada objeto en esa lista
tiene la siguiente estructura.

[inline-code-attrs-start title = 'El objeto de menciones de Comment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** El id del usuario. Para usuarios SSO, esto tendrá prefijado tu id de tenant. **/
    id: string
    /** El texto final de la etiqueta @mention, incluyendo el símbolo @. **/
    tag: string
    /** El texto original de la etiqueta @mention, incluyendo el símbolo @. **/
    rawTag: string
    /** Qué tipo de usuario fue etiquetado. user = cuenta de FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Si el usuario opta por no recibir notificaciones, esto aún se establecerá en true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Cuando se usan hashtags y se analizan correctamente, la información se almacena en una lista llamada `hashTags`. Cada objeto en esa lista
tiene la siguiente estructura. Los hashtags también pueden añadirse manualmente al array `hashTags` del comentario para consultas, si se establece `retain`.

[inline-code-attrs-start title = 'El objeto HashTag de Comment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** El id del hashtag. **/
    id: string
    /** El texto final de la etiqueta #hashtag, incluyendo el símbolo #. **/
    tag: string
    /** Si el hashtag está asociado con una URL personalizada, esto estará definido. **/
    url?: string
    /** Si debemos conservar el hashtag, incluso si no existe en el texto del comentario, cuando el comentario se actualice. Útil para etiquetar comentarios sin cambiar el texto del comentario. **/
    retain?: boolean
}
[inline-code-end]

---