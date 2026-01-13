[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Этот API использует пагинацию, задаваемую параметром запроса `page`. EmailTemplates возвращаются страницами по `100`, отсортированными по `createdAt`, а затем по `id`.

[inline-code-attrs-start title = 'Пример cURL-запроса EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Страница для получения, нумерация начинается с 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Указывается при ошибке. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]

---