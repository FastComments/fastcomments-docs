[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users'; creditsCost = 10; api-resource-header-end]

Denne rute returnerer SSO-brugere i sider af `100`. Paginering leveres via `skip`-parameteren. Brugere sorteres efter deres `signUpDate` og `id`.

[inline-code-attrs-start title = 'SSOUsers cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUsers Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUsers Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    users: SSOUser[]
}
[inline-code-end]
