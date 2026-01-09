[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

В настоящее время вы можете получить только все страницы (или одну страницу через `/by-url-id`), связанные с вашей учетной записью. Если вам нужен более точный поиск, [свяжитесь с нами](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Пример cURL-запроса для страниц'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса для страниц'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа для страниц'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Присутствует в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Присутствует в случае ошибки. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Полезный совет

The `Comment` API requires a `urlId`. You can call the `Pages` API first, to see what the `urlId` values available to you
look like.