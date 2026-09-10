[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Describe la credencial que realiza la solicitud: el inquilino al que pertenece y, para tokens de acceso OAuth, el usuario que autorizó la aplicación. Las integraciones lo utilizan para probar una conexión y etiquetarla.

Con una clave API, la respuesta identifica solo al inquilino. Con un token portador OAuth también incluye al usuario que autoriza y los alcances que se concedieron.

[inline-code-attrs-start title = 'Ejemplo cURL de Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluido en caso de error. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** Cómo se autenticó la solicitud. **/
    authType: 'api-key' | 'oauth'
    /** Los alcances que posee la credencial. Las claves API poseen ambos. **/
    scopes: ('read' | 'write')[]
    /** Solo presente para tokens OAuth. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]