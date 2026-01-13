Un objeto `NotificationCount` representa el conteo de notificaciones no leídas y metadatos para un usuario.

Si no hay notificaciones no leídas, no habrá un `NotificationCount` para el usuario.

Los objetos `NotificationCount` se crean automáticamente y no pueden crearse vía la API. También expiran después de un año.

Puede limpiar el conteo de notificaciones no leídas de un usuario eliminando su `NotificationCount`.

La estructura del objeto `NotificationCount` es la siguiente:

[inline-code-attrs-start title = 'Estructura de NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]
