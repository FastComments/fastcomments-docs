[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

このルートは単一の `TenantUser` を置換する機能を提供します。

`TenantUser` の置換には次の制限があります:

- `signUpDate` は未来の日付にできません。
- `locale` は [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) の一覧に含まれている必要があります。
- `username` は FastComments.com 全体で一意である必要があります。これが問題になる場合は、代わりに SSO の使用を推奨します。
- `email` は FastComments.com 全体で一意である必要があります。これが問題になる場合は、代わりに SSO の使用を推奨します。
- ユーザーの `tenantId` を更新することはできません。

次のように `TenantUser` を置換できます

[inline-code-attrs-start title = 'TenantUser 置換の cURL 例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 置換のリクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** email または username が変更されたとき、これを true に設定するとユーザーのコメントも更新できます。これによりクレジットコストが2倍になります。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 置換のレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]