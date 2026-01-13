[api-resource-header-start name = 'DomainConfig'; route = 'PATCH /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

Ova API krajnja tačka omogućava ažuriranje konfiguracije domena navodeći samo domen i atribut koji se ažurira.

[inline-code-attrs-start title = 'DomainConfig PATCH cURL Primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"emailFromName": "Some New From Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za DomainConfig PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za DomainConfig PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'domain-does-not-exist' | 'update-would-create-duplicate' | 'domain-too-long' | 'domain-invalid';  
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Ažurirana konfiguracija. **/
    configuration?: DomainConfig
}
[inline-code-end]