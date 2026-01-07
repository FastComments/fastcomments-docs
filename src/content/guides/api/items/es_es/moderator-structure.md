Un objeto `Moderator` representa la configuración para un moderador.

Hay tres tipos de moderadores:

1. Usuarios administradores que tienen la bandera `isCommentModeratorAdmin`.
2. Usuarios SSO con la bandera `isCommentModeratorAdmin`.
3. Comentaristas regulares, o usuarios de FastComments.com, que son invitados como Moderadores.

La estructura `Moderator` se usa para representar el Estado de Moderación del caso de uso `3`.

Si quiere invitar a un usuario a ser moderador, vía la API, use la API de `Moderator` creando un `Moderator` e `invitándolo`.

Si el usuario no tiene una cuenta de FastComments.com, el email de invitación les ayudará a configurarse. Si ya tienen una cuenta, se les dará acceso de moderación a su inquilino y el `userId` del objeto `Moderator` se actualizará para apuntar a su usuario. No tendrá acceso API a su usuario, ya que en este caso les pertenece a ellos mismos y es gestionado por FastComments.com.

Si requiere gestión completa de la cuenta del usuario, recomendamos usar SSO, o agregarlos como un [Usuario de Inquilino](https://fastcomments.com/auth/my-account/users) y
luego agregar un objeto `Moderator` para rastrear sus estadísticas.

La estructura `Moderator` puede usarse como un mecanismo de seguimiento de estadísticas para los casos de uso `1` y `2`. Después de crear el usuario, agregue un objeto `Moderator` con su `userId` definido y sus estadísticas serán rastreadas en la [Página de Moderadores de Comentarios](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

La estructura del objeto `Moderator` es la siguiente:

[inline-code-attrs-start title = 'Estructura de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]
