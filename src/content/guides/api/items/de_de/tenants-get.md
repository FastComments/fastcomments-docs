[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Diese API gibt Tenants zurück, die von Ihrem Tenant verwaltet werden.

Die Paginierung wird durch den `skip`-Abfrageparameter bereitgestellt. Tenants werden in Seiten von `100` zurückgegeben, sortiert nach `signUpDate` und `id`.

Die Kosten basieren auf der Anzahl der zurückgegebenen Tenants und betragen `1 Credit pro 10` zurückgegebene Tenants.

[inline-code-attrs-start title = 'Tenant cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Sie können `meta`-Parameter für die `Tenant`-Objekte definieren und nach passenden Tenants abfragen. Zum Beispiel können wir für den Schlüssel `someKey` und den Meta-Wert `some-value`
ein JSON-Objekt mit diesem Schlüssel/Wert-Paar erstellen und es dann als Abfrageparameter URI-kodieren, um zu filtern:

[inline-code-attrs-start title = 'Tenant Abfrage nach Meta cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
    /** Filter by meta params. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
