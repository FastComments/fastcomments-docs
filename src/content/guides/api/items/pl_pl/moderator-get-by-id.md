[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

Ten endpoint zwraca pojedynczego moderatora na podstawie jego/jej id.

[inline-code-attrs-start title = 'Przykład cURL — Moderator według ID'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorByIdResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    moderator?: Moderator
}
[inline-code-end]

---