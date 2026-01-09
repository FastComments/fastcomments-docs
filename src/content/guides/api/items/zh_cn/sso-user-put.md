[api-resource-header-start name = 'SSOUser'; route = 'PUT /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

此路由提供更新单个 SSO 用户的功能。

[inline-code-attrs-start title = 'SSOUser 更新 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/sso-users/my-user-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

在此示例中，我们为访问控制指定了 `groupIds`，但这是可选的。

[inline-code-attrs-start title = 'SSOUser 更新 请求 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 当更改 email 或 username 时，您可以将此设置为 true 以同时更新该用户的评论。这将使费用翻倍。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 更新 响应 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPutResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** 失败时包含。 **/
    reason?: string
    user?: SSOUser; // 成功时返回更新的用户。
}
[inline-code-end]


---