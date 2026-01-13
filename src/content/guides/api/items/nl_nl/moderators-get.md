[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Deze API gebruikt paginering via de queryparameter `skip`. Moderators worden geretourneerd in pagina's van `100`, gesorteerd op `createdAt` en `id`.

De kosten zijn gebaseerd op het aantal geretourneerde moderators; het kost `1 credit per 10` geretourneerde moderators.

[inline-code-attrs-start title = 'Moderator cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Het aantal moderators om over te slaan voor paginering. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij mislukking. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---