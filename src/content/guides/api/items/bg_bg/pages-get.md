[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

В момента можете да извличате само всички страници (или единична страница чрез `/by-url-id`), свързани с вашия акаунт. Ако искате по-прецизно търсене, [свържете се с нас](https://fastcomments.com/auth/my-account/help).

[inline-code-attrs-start title = 'Пример за Pages с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Полезен съвет

API `Comment` изисква `urlId`. Можете първо да извикате API `Pages`, за да видите как изглеждат наличните за вас стойности на `urlId`.
