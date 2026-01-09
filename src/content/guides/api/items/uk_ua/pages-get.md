[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Зараз ви можете отримати лише всі сторінки (або одну сторінку через `/by-url-id`), пов'язані з вашим обліковим записом. Якщо ви хочете здійснювати більш детальний пошук, [зв'яжіться з нами](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Приклад cURL-запиту для Pages'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається у разі помилки. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Корисна порада

API `Comment` вимагає `urlId`. Ви можете спочатку викликати API `Pages`, щоб побачити, як виглядають доступні вам значення `urlId`.