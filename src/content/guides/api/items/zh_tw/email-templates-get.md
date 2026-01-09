[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

此 API 使用分頁，由 `page` 查詢參數提供。EmailTemplates 以每頁 `100` 筆的方式回傳，排序先依 `createdAt`，再依 `id`。

[inline-code-attrs-start title = 'EmailTemplate cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 要擷取的頁面，從 0 開始。 **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時會包含。 **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]