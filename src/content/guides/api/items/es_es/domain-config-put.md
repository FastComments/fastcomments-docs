[api-resource-header-start name = 'DomainConfig'; route = 'PUT /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de reemplazar una configuración de dominio.

[inline-code-attrs-start title = 'Ejemplo cURL de PUT de DomainConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"domain": "new-domain-example.com",
	"emailFromName": "some from name",
	"emailFromEmail": "some@test.com",
	"logoSrc": "https://example.com/my-logo-big.png",
	"logoSrc100px": "https://example.com/my-logo-100px.png",
	"footerUnsubscribeURL": "http://example.com/unsubscribe-ui",
	"emailHeaders": {
		"List-Unsubscribe-Post": "List-Unsubscribe=One-Click",
		"List-Unsubscribe": "<https://example.com/opt-out/[userId]>"
	}
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de PUT de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPutQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de PUT de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'domain-does-not-exist' | 'update-would-create-duplicate' | 'domain-too-long' | 'domain-invalid';
    /** Included on failure. **/
    reason?: string
    /** The updated configuration. **/
    configuration?: DomainConfig
}
[inline-code-end]
