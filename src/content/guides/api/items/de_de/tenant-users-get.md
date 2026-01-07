[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Diese API verwendet Paginierung, bereitgestellt durch den `skip`-Abfrageparameter. TenantUsers werden in Seiten von `100` zurückgegeben, sortiert nach `signUpDate`, `username` und `id`.

Die Kosten basieren auf der Anzahl der zurückgegebenen Tenant-Benutzer und betragen `1 Credit pro 10` zurückgegebene Tenant-Benutzer.

[inline-code-attrs-start title = 'TenantUser cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant users to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]
