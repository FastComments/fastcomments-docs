 [api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la capacidad de agregar un único `Vote` autorizado. Los votos pueden ser `up` (+1) o `down` (-1).

[inline-code-attrs-start title = 'Ejemplo cURL de Creación de Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Ejemplo cURL de Creación de Vote Anónimo'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Creación de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Creación de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Creación de Votos Anónimos

Los votos anónimos pueden ser creados estableciendo `anonUserId` en los parámetros de consulta en lugar de `userId`.

Este id no tiene que corresponder a un objeto de usuario en ningún lugar (de ahí anónimo). Es simplemente un identificador
para la sesión, para que pueda obtener los votos nuevamente en la misma sesión, para verificar si un comentario ha
sido votado.

Si no tiene algo como "sesiones anónimas" como FastComments - simplemente puede
establecer esto en un ID aleatorio, como un UUID (aunque apreciamos identificadores más pequeños para ahorrar espacio).

### Otras Notas

- Esta API obedece la configuración a nivel de inquilino. Por ejemplo, si deshabilita la votación para una página determinada, e intenta crear un voto a través de la API, fallará con el código de error `voting-disabled`.
- Esta API es en vivo por defecto.
- Esta API actualizará los `votes` del `Comment` correspondiente.
