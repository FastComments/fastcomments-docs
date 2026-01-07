[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Denne API bruger paginering, leveret af `skip` query-parameteren. Moderatorer returneres i sider af `100`, sorteret efter `createdAt` og `id`.

Prisen er baseret på antallet af returnerede moderatorer, koste `1 kredit pr. 10` returnerede moderatorer.

[inline-code-attrs-start title = 'Moderator cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of moderators to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]
