[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

このルートは単一の `TenantUser` を追加する機能を提供します。

`TenantUser` を作成する際には、以下の制約があります:

- `username` は必須です。
- `email` は必須です。
- `signUpDate` は未来の日付にできません。
- `locale` は [サポートされているロケール](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) の一覧に含まれている必要があります。
- `username` は FastComments.com 上で一意である必要があります。もし問題がある場合は、代わりに SSO の使用を推奨します。
- `email` は FastComments.com 上で一意である必要があります。もし問題がある場合は、代わりに SSO の使用を推奨します。
- パッケージで定義されている `maxTenantUsers` を超えてテナントユーザーを作成することはできません。 

次のように `TenantUser` を作成できます

[inline-code-attrs-start title = 'TenantUser 作成の cURL 例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 作成リクエストの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 作成レスポンスの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** 失敗時に含まれます。 **/
    reason?: string
    tenantUser?: TenantUser; // 成功時に作成された完全な tenantUser を返します。
}
[inline-code-end]

---