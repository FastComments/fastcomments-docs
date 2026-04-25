[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端点提供更新单条评论的功能。

说明：

- 如果需要，此 API 可以“实时”更新评论小部件（这会将基础 `creditsCost` 从 `1` 增加到 `2`）。
  - 这可以使在页面之间迁移评论变为“实时”（更改 `urlId`）。
  - 迁移会额外消耗 `2` 个积分，因为页面会被预计算，这会消耗大量 CPU。
- 与创建 API 不同，如果提供了电子邮件，此 API 不会在我们的系统中自动创建用户对象。
- 通过此 API 更新的评论仍然可以根据需要进行垃圾评论检查。
- 如果通过“自定义规则”管理页面配置了诸如最大评论长度等设置，那么这些配置在此处仍然适用。
- 若要允许用户更新其评论文本，您只需在请求体中指定 `comment`。我们将生成相应的 `commentHTML`。
  - 如果同时定义了 `comment` 和 `commentHTML`，我们将不会自动生成 HTML。
  - 如果用户在新文本中添加了提及或标签，它仍将像 `POST` API 一样进行处理。
- 在更新评论的 `commenterEmail` 时，最好也指定 `userId`。否则，您必须确保具有该电子邮件的用户属于您的租户，否则请求将失败。
- 如果目标评论被锁定（`isLocked: true`），请求将以 `code: 'locked'` 被拒绝。请先解锁评论，进行更新，然后在需要时重新锁定。


[inline-code-attrs-start title = '最小评论 PATCH cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = '评论 PATCH 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** 执行更新的用户。可用于检查他们是否可以编辑该评论。 **/
    contextUserId?: string
	/** 是否应该检查新评论是否看起来像垃圾评论？ **/
    doSpamCheck?: 'true' | 'false'
	/** 评论是否应对查看具有相同 urlId 的评论小部件实例的用户“实时”显示。NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '评论 PATCH 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]