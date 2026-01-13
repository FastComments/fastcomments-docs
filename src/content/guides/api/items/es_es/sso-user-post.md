[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

Esta ruta proporciona la creación de un único usuario SSO.

Intentar crear dos usuarios con el mismo ID resultará en un error.

[inline-code-attrs-start title = 'Ejemplo cURL de Creación de SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

En este ejemplo especificamos `groupIds` para control de acceso, pero esto es opcional.

[inline-code-attrs-start title = 'Estructura de Solicitud de Creación de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Creación de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### Nota de Integración

Los datos pasados por la API pueden ser sobrescritos simplemente pasando un payload de Usuario SSO HMAC diferente. Por ejemplo, si
establece un nombre de usuario vía la API, pero luego pasa uno diferente vía el flujo SSO al cargar la página, actualizaremos automáticamente
su nombre de usuario.

No actualizaremos parámetros de usuario en este flujo a menos que los especifique explícitamente o los establezca a null (no undefined).
