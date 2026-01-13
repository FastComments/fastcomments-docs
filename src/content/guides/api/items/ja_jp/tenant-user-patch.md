[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

このルートは単一の `TenantUser` を更新する機能を提供します。

`TenantUser` を更新する際の制限は次のとおりです:

- `signUpDate` は未来の日付にできません。
- `locale` は [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) の一覧に含まれている必要があります。
- `username` は FastComments.com 全体で一意である必要があります。問題がある場合は、代わりに SSO の使用を推奨します。
- `email` は FastComments.com 全体で一意である必要があります。問題がある場合は、代わりに SSO の使用を推奨します。
- ユーザーの `tenantId` を更新することはできません。

次のように `TenantUser` を作成できます

[inline-code-attrs-start title = 'TenantUser 更新 cURL の例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 更新リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** email または username が変更されたとき、これを 'true' に設定するとユーザーのコメントも更新できます。これによりクレジットコストが2倍になります。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 更新レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]