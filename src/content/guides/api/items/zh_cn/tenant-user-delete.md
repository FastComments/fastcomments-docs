[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

此路由根据 id 提供删除 `TenantUser` 的功能。

可以通过 `deleteComments` 查询参数同时删除该用户的评论。请注意，如果设置为 true：

1. 该用户的所有评论将会被实时删除。
2. 所有 __child__（现在为孤立）的评论将根据每个评论关联页面的配置被删除或匿名化。例如，如果线程删除模式为 "anonymize"，则回复将保留，而该用户的评论将被匿名化。仅当 `commentDeleteMode` 为 `Remove`（默认值）时适用。
3. `creditsCost` 会变为 `2`。

### 匿名化的评论

您可以保留该用户的评论，但通过设置 `commentDeleteMode=1` 将其匿名化。

如果该用户的评论被匿名化，则以下值将被设置为 null：

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` 和 `isDeletedUser` 被设置为 `true`。

在渲染时，评论小部件会使用 `DELETED_USER_PLACEHOLDER`（默认: "[deleted]"）作为用户名，使用 `DELETED_CONTENT_PLACEHOLDER` 作为评论内容。这些可以通过小部件自定义界面进行定制。

### 示例

[inline-code-attrs-start title = 'TenantUser 删除 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 删除 请求 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // 默认
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** 您可以将此设置为 true 以同时删除该用户的评论。这样会使信用消耗加倍。 **/
    deleteComments?: 'true' | 'false'
    /** 您可以按需设置此项以确定如何处理该用户的评论。 **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 删除 响应 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]