[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Bu API, `skip` sorgu parametresi tarafından sağlanan sayfalandırmayı kullanır. Moderators, `createdAt` ve `id`'ye göre sıralanmış olarak `100`lük sayfalar halinde döndürülür.

Maliyet, döndürülen moderatör sayısına göre belirlenir; döndürülen moderatörler için maliyet `1 credit per 10`'dur.

[inline-code-attrs-start title = 'Moderator cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Sayfalandırma için atlanacak moderatör sayısı. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---