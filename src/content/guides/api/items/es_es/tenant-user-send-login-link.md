[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Esta ruta proporciona la capacidad de enviar un enlace de inicio de sesión a un único `TenantUser`.

Útil al crear usuarios por lotes sin tener que instruirles sobre cómo iniciar sesión en FastComments.com. Esto simplemente les enviará un "enlace mágico" para iniciar sesión que
expira después de `30 días`.

Las siguientes restricciones existen para enviar un enlace de inicio de sesión a un `TenantUser`:
- El `TenantUser` ya debe existir.
- Debe tener acceso para administrar el `Tenant` al que pertenece el `TenantUser`.

Podemos enviar un enlace de inicio de sesión a un `TenantUser` de la siguiente manera:

[inline-code-attrs-start title = 'Ejemplo cURL de Enlace de Inicio de Sesión de TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Esto enviará un email como `Bob en TenantName te está invitando a ser moderador...`

[inline-code-attrs-start title = 'Estructura de Solicitud de Enlace de Inicio de Sesión de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Enlace de Inicio de Sesión de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
