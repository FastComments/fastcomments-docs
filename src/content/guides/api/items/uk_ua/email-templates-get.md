[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Цей API використовує пагінацію, яка задається параметром запиту `page`. EmailTemplates повертаються сторінками по `100`, відсортовані за `createdAt`, а потім за `id`.

[inline-code-attrs-start title = 'Приклад cURL для EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Номер сторінки для отримання, починаючи з 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Включено у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включено у разі невдачі. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]

---