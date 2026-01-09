[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Овај API користи пагинацију, обезбијеђену параметром упита `page`. EmailTemplates се враћају у страницама по `100`, уређене по `createdAt`, а затим по `id`.

[inline-code-attrs-start title = 'Примјер cURL за EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Страница за преузимање, почиње од 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]

---