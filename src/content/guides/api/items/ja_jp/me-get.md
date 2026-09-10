[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

リクエストを行う認証情報を記述します：所属するテナントと、OAuth アクセストークンの場合はアプリケーションを認可したユーザーです。統合はこれを使用して接続をテストし、ラベル付けします。

API キーを使用した場合、レスポンスはテナントのみを識別します。OAuth ベアラートークンを使用した場合、認可されたユーザーと付与されたスコープも含まれます。

[inline-code-attrs-start title = 'Me cURL例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Meレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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