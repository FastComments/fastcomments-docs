[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Denne API-endpoint giver mulighed for at opdatere en e-mail-skabelon ved kun at angive id'et og attributterne, der skal opdateres.

Bemærk at alle de samme valideringer for oprettelse af en skabelon også gælder, for eksempel:

- Skabelonen skal kunne renderes. Dette kontrolleres ved hver opdatering.
- Du kan ikke have duplikerede skabeloner for det samme domæne (ellers ville en blive ignoreret lydløst).

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The updated email template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]
