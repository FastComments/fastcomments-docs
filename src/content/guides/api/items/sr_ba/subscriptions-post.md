[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ova API krajnja tačka omogućava kreiranje `Subscription`-a. Imajte na umu da korisnik može imati samo jednu pretplatu po stranici, jer više njih je suvišno, i pokušaj kreiranja više od jedne pretplate za istog korisnika na istoj stranici rezultirat će greškom.

Kreiranje pretplate će rezultirati stvaranjem objekata `Notification` kada se novi komentar ostavi na korijenu pretplaćenog `urlId` (kada je `parentId` komentara `null`).

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za POST Subscription'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST zahtjeva za Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura POST odgovora za Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Uključeno pri neuspjehu. **/
    reason?: string
}
[inline-code-end]

---