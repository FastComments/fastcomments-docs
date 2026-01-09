[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ovaj API koristi paginaciju, koju obezbeđuje parametar upita `skip`. Moderatori se vraćaju u stranicama po `100`, poređani po `createdAt` i `id`.

Trošak se zasniva na broju vraćenih moderatora: `1 credit per 10` moderatora.

[inline-code-attrs-start title = 'cURL primer za Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Moderatore'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Broj moderatora koji se preskače za paginaciju. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Moderatore'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---