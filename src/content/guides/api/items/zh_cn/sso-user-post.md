[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

此路由用于创建单个 SSO 用户。

尝试创建两个具有相同 ID 的用户将导致错误。

[inline-code-attrs-start title = 'SSOUser 创建 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

在此示例中，我们为访问控制指定了 `groupIds`，但这是可选的。

[inline-code-attrs-start title = 'SSOUser 创建请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 创建响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** 失败时包含。 **/
    reason?: string
    user?: SSOUser; // 成功时我们返回创建的用户。
}
[inline-code-end]

#### 集成说明

通过 API 传递的数据可以被覆盖，只需传递一个不同的 SSO 用户 HMAC 载荷即可。例如，如果您通过 API 设置了用户名，但在页面加载时通过 SSO 流程传递了不同的用户名，我们将自动更新该用户的用户名。

在此流程中，除非您显式指定用户参数或将其设置为 null（而非 undefined），否则我们不会更新用户参数。