[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ta končna točka vrne do 30 objektov `Subscription` urejenih po `createdAt`, najnovejši prvi.

Filtrirate lahko po `userId`. Pri SSO je uporabniški id v formatu `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Primer cURL za naročnine uporabnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za naročnine'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Straničenje s preskakovanjem zapisov. **/
    skip?: number
    /** Filtriraj po uporabniku. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za naročnine'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Vključeno ob neuspehu. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---