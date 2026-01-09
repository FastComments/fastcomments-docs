[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Ovaj API koristi paginaciju, koja je obezbeđena `skip` query parametrom. Moderatori se vraćaju u stranicama po `100`, sortirani po `createdAt` i `id`.

Cena se zasniva na broju vraćenih moderatora, i iznosi `1 credit per 10` vraćenih moderatora.

[inline-code-attrs-start title = 'Primer cURL zahteva za moderatora'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Broj moderatora koje treba preskočiti za paginaciju. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Uključen u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključen u slučaju neuspeha. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---