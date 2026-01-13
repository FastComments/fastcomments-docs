Un objeto `Subscription` representa una suscripción para un usuario.

Los objetos `Subscription` se crean cuando un usuario hace clic en la campana de notificación en el widget de comentarios y hace clic en "Suscribirse a esta página".

Las suscripciones también pueden crearse vía la API.

Tener un objeto `Subscription` causa que se generen objetos `Notification`, y se envíen emails, cuando se dejan nuevos comentarios en la raíz de la página asociada
para la cual es la `Subscription`. El envío de emails depende del tipo de usuario. Para usuarios regulares esto depende de `optedInNotifications`. Para Usuarios SSO esto depende de `optedInSubscriptionNotifications`. Tenga en cuenta que algunas aplicaciones pueden no tener el concepto de una página accesible por web, en cuyo caso simplemente establezca `urlId` al
id del elemento al que se está suscribiendo (mismo valor para `urlId` que pasaría al widget de comentarios).

La estructura del objeto `Subscription` es la siguiente:

[inline-code-attrs-start title = 'Estructura de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]
