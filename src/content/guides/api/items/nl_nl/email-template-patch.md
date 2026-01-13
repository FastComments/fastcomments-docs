[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt biedt de mogelijkheid om een e-mailtemplate bij te werken door alleen de id en de attributen die bijgewerkt moeten worden op te geven.

Houd er rekening mee dat alle validaties die gelden bij het aanmaken van een template ook van toepassing zijn, bijvoorbeeld:

- De template moet renderen. Dit wordt bij elke update gecontroleerd.
- Er mogen geen dubbele templates voor hetzelfde domein bestaan (anders wordt er één stilzwijgend genegeerd).

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Opgenomen bij een fout. **/
    reason?: string
    /** Het bijgewerkte e-mailtemplate. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]