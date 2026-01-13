[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

此路由根据其 id 删除单个 SSO 用户。

请注意，如果使用该用户的有效负载重新加载评论小部件，系统会无缝地重新创建该用户。

可以通过查询参数 `deleteComments` 删除该用户的评论。请注意，如果该值为 true：

1. 该用户的所有评论将被实时删除。
2. 所有 __child__（现在成为孤儿）的评论将根据每个评论关联页面的配置被删除或匿名化。例如，如果线程删除模式为 "anonymize"，则回复将保留，而该用户的评论将被匿名化。仅当 `commentDeleteMode` 为 `Remove`（默认值）时适用。
3. `creditsCost` 将变为 `2`。

### 匿名化的评论

您可以保留该用户的评论，但通过设置 `commentDeleteMode=1` 将其匿名化。

如果该用户的评论被匿名化，则以下字段将被设置为 null：

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` 和 `isDeletedUser` 将被设置为 `true`。

在渲染时，评论小部件将在用户名称处使用 `DELETED_USER_PLACEHOLDER`（默认值: "[deleted]"）和在评论处使用 `DELETED_CONTENT_PLACEHOLDER`。这些可以通过小部件自定义界面进行自定义。

### 示例

[inline-code-attrs-start title = 'SSOUser 删除 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 删除请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // 默认
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** 您可以将此设置为 true 以同时删除该用户的评论。 这会使消耗的 credits 翻倍。 **/
    deleteComments?: 'true' | 'false'
    /** 您可以根据需要设置此项以确定如何处理该用户的评论。 **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 删除响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** 失败时包含。 **/
    reason?: string
    user?: SSOUser; // 成功时返回被移除的用户。
}
[inline-code-end]

---