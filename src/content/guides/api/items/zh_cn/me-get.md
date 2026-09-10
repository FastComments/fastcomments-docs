[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

描述发起请求的凭证：它所属的租户，以及对于 OAuth 访问令牌，授权该应用的用户。集成使用它来测试连接并对其进行标记。

使用 API 密钥时，响应仅标识租户。使用 OAuth 持有者令牌时，还会携带授权用户以及已授予的作用域。

[inline-code-attrs-start title = 'Me cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** How the request was authenticated. **/
    authType: 'api-key' | 'oauth'
    /** The scopes the credential holds. API keys hold both. **/
    scopes: ('read' | 'write')[]
    /** Only present for OAuth tokens. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]