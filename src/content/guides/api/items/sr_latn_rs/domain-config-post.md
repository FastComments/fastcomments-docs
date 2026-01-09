[api-resource-header-start name = 'DomainConfig'; route = 'POST /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava kreiranje konfiguracija domena.

Dodavanje konfiguracije za domen ovlašćuje taj domen za FastComments nalog.

Uobičajene upotrebe ovog API-ja su početno podešavanje, ako je potrebno dodati mnogo domena, ili prilagođena konfiguracija za slanje e-pošte. 

[inline-code-attrs-start title = 'DomainConfig POST cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"domain": "example.com",
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

[inline-code-attrs-start title = 'Struktura POST zahteva za DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST odgovora za DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'configuration-exists-for-domain' | 'domain-too-long' | 'domain-invalid';  
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Kreirana konfiguracija. **/
    configuration?: DomainConfig
}
[inline-code-end]

---