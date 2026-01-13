[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava kreiranje `Subscription`. Imajte na umu da korisnik može imati samo jednu pretplatu po stranici, jer je više njih suvišno, a pokušaj kreiranja više od jedne pretplate za istog korisnika na istoj stranici rezultiraće greškom.

Kreiranje pretplate će rezultirati kreiranjem `Notification` objekata kada se na korijenu pretplaćenog `urlId` ostavi novi komentar (kada je komentar `parentId` `null`).

[inline-code-attrs-start title = 'Primjer POST cURL za Subscription'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]