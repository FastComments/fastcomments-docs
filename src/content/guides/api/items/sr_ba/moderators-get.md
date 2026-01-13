[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Овај API користи пагинацију, коју обезбјеђује параметар упита `skip`. Модератори се враћају у страницама по `100`, сортирани по `createdAt` и `id`.

Трошак се заснива на броју враћених модератора и износи `1 credit per 10` за сваких 10 враћених модератора.

[inline-code-attrs-start title = 'Пример cURL захтева за Модератора'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за Модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Број модератора које треба прескочити за пагинацију. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за Модератора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспеха. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]