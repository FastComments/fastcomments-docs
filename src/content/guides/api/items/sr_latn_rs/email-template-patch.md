[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava ažuriranje email šablona navodeći samo id i atribute koje treba ažurirati.

Imajte na umu da se sve iste validacije kao pri kreiranju šablona takođe primenjuju, na primer:

- Šablon mora da se renderuje. Ovo se proverava pri svakom ažuriranju.
- Ne možete imati duplikate šablona za isti domen (inače bi jedan bio tiho zanemaren).

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura PATCH zahteva za EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za EmailTemplate PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Ažurirani email šablon. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]