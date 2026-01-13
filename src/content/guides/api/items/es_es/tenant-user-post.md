[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la capacidad de agregar un único `TenantUser`.

Crear un `TenantUser` tiene las siguientes restricciones:

- Se requiere un `username`.
- Se requiere un `email`.
- El `signUpDate` no puede estar en el futuro.
- El `locale` debe estar en la lista de [Locales Soportados](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- El `username` debe ser único en todo FastComments.com. Si esto es un problema, sugerimos usar SSO en su lugar.
- El `email` debe ser único en todo FastComments.com. Si esto es un problema, sugerimos usar SSO en su lugar.
- No puede crear más usuarios de inquilino de los definidos en `maxTenantUsers` en su paquete.

Podemos crear un `TenantUser` de la siguiente manera

[inline-code-attrs-start title = 'Ejemplo cURL de Creación de TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Creación de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Creación de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]
