[api-resource-header-start name = 'EmailTemplate'; route = 'DELETE /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

此路由會刪除單一 `EmailTemplate`（依 id）。

[inline-code-attrs-start title = 'EmailTemplate 刪除 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate 刪除請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate 刪除回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateDeleteResponse {
    status: 'success' | 'failed'
    /** 僅在失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** 僅在失敗時包含。 **/
    reason?: string
}
[inline-code-end]

---