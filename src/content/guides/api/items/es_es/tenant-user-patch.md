[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la capacidad de actualizar un único `TenantUser`.

Actualizar un `TenantUser` tiene las siguientes restricciones:

- El `signUpDate` no puede estar en el futuro.
- El `locale` debe estar en la lista de [Locales Soportados](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- El `username` debe ser único en todo FastComments.com. Si esto es un problema, sugerimos usar SSO en su lugar.
- El `email` debe ser único en todo FastComments.com. Si esto es un problema, sugerimos usar SSO en su lugar.
- No puede actualizar el `tenantId` de un usuario.

Podemos crear un `TenantUser` de la siguiente manera

[inline-code-attrs-start title = 'Ejemplo cURL de Actualización de TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Actualización de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Actualización de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
