[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint vraća do 30 `Subscription` objekata sortiranih po `createdAt`, najnovije prvo.

Možete filtrirati po `userId`. Uz SSO, id korisnika je u formatu `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'cURL primjer pretplata za korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za pretplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginacija preskakanjem zapisa. **/
    skip?: number
    /** Filtriraj po korisniku. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za pretplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]