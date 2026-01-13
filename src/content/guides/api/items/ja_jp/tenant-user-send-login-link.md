[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

このルートは単一の `TenantUser` にログインリンクを送信する機能を提供します。

ユーザーを一括作成する際に、FastComments.com へのログイン方法を個別に案内する必要がない場合に便利です。これはログイン用の「マジックリンク」を送信し、そのリンクは `30 days` で期限切れになります。

`TenantUser` にログインリンクを送信するための制限は以下のとおりです：
- `TenantUser` は既に存在している必要があります。
- `TenantUser` が所属する `Tenant` を管理するアクセス権が必要です。

`TenantUser` にログインリンクを送信する例は次のとおりです：

[inline-code-attrs-start title = 'TenantUser ログインリンク cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

これにより `Bob at TenantName is inviting you to be a moderator...` のようなメールが送信されます。

[inline-code-attrs-start title = 'TenantUser ログインリンク リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser ログインリンク レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]