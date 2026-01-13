[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Deze route retourneert maximaal 30 `Subscription`-objecten, gesorteerd op `createdAt`, van nieuw naar oud.

Je kunt filteren op `userId`. Bij SSO is de gebruikers-id in het formaat `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'cURL-voorbeeld: abonnementen voor gebruiker'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het abonnementenverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Pagineren door records over te slaan. **/
    skip?: number
    /** Filter op gebruiker. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de abonnementenrespons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Opgenomen bij mislukking. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---