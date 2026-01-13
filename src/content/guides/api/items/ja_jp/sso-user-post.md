[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

このルートは単一のSSOユーザーを作成します。

同じIDでユーザーを2つ作成しようとするとエラーになります。

[inline-code-attrs-start title = 'SSOUser 作成の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

この例ではアクセス制御のために `groupIds` を指定していますが、これはオプションです。

[inline-code-attrs-start title = 'SSOUser 作成リクエストの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 作成レスポンスの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** 失敗時に含まれます。 **/
    reason?: string
    user?: SSOUser; // 成功時には作成したユーザーを返します。
}
[inline-code-end]

#### 統合に関する注意

APIによって渡されたデータは、別のSSOユーザHMACペイロードを渡すだけで簡単に上書きできます。例えば、APIでusernameを設定した場合でも、ページ読み込み時のSSOフローで別のusernameを渡すと、自動的にそのusernameが更新されます。

このフローでは、明示的に指定するか null（not undefined）に設定した場合を除き、ユーザーのパラメータは更新しません。

---