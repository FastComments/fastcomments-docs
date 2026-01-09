[api-resource-header-start name = 'SSOUser'; route = 'PATCH /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

此路由提供更新單一 SSO 使用者的功能。

[inline-code-attrs-start title = 'SSOUser 更新 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "notfordperfect"
}'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 更新請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** 當電子郵件或使用者名稱被更改時，您可以將此設定為 true 以同時更新該使用者的評論。這會使信用成本增加一倍。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 更新回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPatchResponse {
    status: 'success' | 'failed'
    /** 在失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-does-not-exist'
    /** 在失敗時會包含。 **/
    reason?: string
    user?: SSOUser; // 在成功時會回傳完整更新後的使用者。
}
[inline-code-end]