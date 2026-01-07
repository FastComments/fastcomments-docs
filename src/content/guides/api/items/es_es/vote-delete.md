[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la capacidad de eliminar un único `Vote`.

[inline-code-attrs-start title = 'Ejemplo cURL de Eliminación de Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Eliminación de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Eliminación de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

Notas:

- Esta API obedece la configuración a nivel de inquilino. Por ejemplo, si deshabilita la votación para una página determinada, e intenta crear un voto a través de la API, fallará con el código de error `voting-disabled`.
- Esta API es en vivo por defecto.
- Esta API actualizará los `votes` del `Comment` correspondiente.
