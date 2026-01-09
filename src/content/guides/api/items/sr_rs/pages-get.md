[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Тренутно можете да преузмете само све странице (или једну страницу путем `/by-url-id`) повезане са вашим налогом. Ако желите прецизније претраживање, [контактирајте нас](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Пример cURL за странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспеха. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Корисан савет

API `Comment` захтева `urlId`. Можете прво позвати `Pages` API да бисте видели како изгледају вредности `urlId` које су вам доступне.