[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Вы можете в настоящее время получать только все страницы (или одну страницу через `/by-url-id`), связанные с вашей учетной записью. Если вам нужна более точная фильтрация при поиске, [свяжитесь с нами](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Пример cURL-запроса для Pages'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса для Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа для Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включается при ошибке. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Полезный совет

API `Comment` требует `urlId`. Вы можете сначала вызвать API `Pages`, чтобы посмотреть, какие значения `urlId` доступны вам
выглядят.