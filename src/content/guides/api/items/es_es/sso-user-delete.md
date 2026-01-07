[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la eliminación de un único usuario SSO por su id.

Tenga en cuenta que cargar el widget de comentarios nuevamente con un payload para este usuario simplemente recreará el usuario sin problemas.

Eliminar los comentarios del usuario es posible vía el parámetro de consulta `deleteComments`. Tenga en cuenta que si esto es verdadero:

1. Todos los comentarios del usuario serán eliminados en vivo.
2. Todos los comentarios __hijos__ (ahora huérfanos) serán eliminados o anonimizados basándose en la configuración de página asociada de cada comentario. Por ejemplo si el modo de eliminación de hilo es "anonimizar", entonces las respuestas permanecerán, y los comentarios del usuario serán anonimizados. Esto solo aplica cuando `commentDeleteMode` es `Remove` (el valor predeterminado).
3. El `creditsCost` se convierte en `2`.

### Comentarios Anonimizados

Puede retener los comentarios del usuario pero simplemente anonimizarlos estableciendo `commentDeleteMode=1`.

Si los comentarios del usuario son anonimizados entonces los siguientes valores se establecen a null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` e `isDeletedUser` se establecen a `true`.

Al renderizar, el widget de comentarios usará `DELETED_USER_PLACEHOLDER` (predeterminado: "[deleted]") para el nombre del usuario y `DELETED_CONTENT_PLACEHOLDER` para el comentario. Estos pueden personalizarse vía la UI de Personalización del Widget.

### Ejemplos

[inline-code-attrs-start title = 'Ejemplo cURL de Eliminación de SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Eliminación de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Eliminación de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the removed user on success.
}
[inline-code-end]
