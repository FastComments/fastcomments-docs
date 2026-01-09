[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućuje ažuriranje predloška e-pošte tako da se navede samo id i atributi koje treba ažurirati.

Imajte na umu da vrijede ista pravila validacije kao i pri stvaranju predloška, na primjer:

- Predložak se mora renderirati. To se provjerava pri svakom ažuriranju.
- Ne možete imati duplicirane predloške za istu domenu (inače bi jedan bio tiho ignoriran).

[inline-code-attrs-start title = 'EmailTemplate PATCH primjer cURL-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Struktura zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH Struktura odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Uključeno pri neuspjehu. **/
    reason?: string
    /** Ažurirani predložak e-pošte. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---