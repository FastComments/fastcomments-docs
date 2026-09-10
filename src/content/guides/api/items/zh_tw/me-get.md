[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

描述發出請求的憑證：它所屬的租戶，以及對於 OAuth 存取權杖，
授權該應用程式的使用者。整合會使用它來測試連線並為其加上標籤。

使用 API 金鑰時，回應僅會識別租戶。使用 OAuth bearer token 時，回應還會攜帶
授權的使用者以及已授予的範圍。

[inline-code-attrs-start title = 'Me cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時包含。 **/
    reason?: string
    tenantId: string
    tenantName: string
    /** 請求的驗證方式。 **/
    authType: 'api-key' | 'oauth'
    /** 憑證所持有的範圍。API 金鑰同時擁有兩者。 **/
    scopes: ('read' | 'write')[]
    /** 僅在 OAuth token 時出現。 **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]

---