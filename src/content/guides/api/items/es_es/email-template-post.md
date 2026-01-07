[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de crear plantillas de email.

Notas:

- No puede tener múltiples plantillas con el mismo `emailTemplateId` con el mismo dominio.
- Pero puede tener una plantilla comodín (`domain` = `*` y una plantilla específica de dominio para el mismo `emailTemplateId`).
- Especificar `domain` solo es relevante si tiene diferentes dominios, o quiere usar plantillas específicas para pruebas (`domain` establecido a `localhost` etc).
- Si especifica `domain` debe coincidir con un `DomainConfig`. En caso de error se proporciona una lista de dominios válidos.
- La sintaxis de plantilla es EJS y se renderiza con un timeout de 500ms. P99 para renderizado es <5ms, así que si llega a 500ms algo está mal.
- **Su plantilla debe renderizarse con su `testData` dado** para guardar. Los errores de renderizado se agregan y reportan en el dashboard (pronto disponible vía API).

Los datos mínimos requeridos para agregar una plantilla son los siguientes:

[inline-code-attrs-start title = 'Ejemplo cURL Mínimo de POST de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

Puede querer tener plantillas por sitio, en cuyo caso define `domain`:

[inline-code-attrs-start title = 'Ejemplo cURL de POST de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de POST de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de POST de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
