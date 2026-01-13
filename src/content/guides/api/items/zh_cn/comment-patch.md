[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端点提供更新单条评论的能力。

注意：

- 如果需要，此 API 可以使评论小部件“实时”更新（这会将基础 `creditsCost` 从 `1` 提升到 `2`）。
  - 这可以使在页面之间迁移评论时实时生效（更改 `urlId`）。
  - 迁移会额外消耗 `2` 学分，因为页面需要预计算且这很耗费 CPU。
- 与创建 API 不同，如果提供了电子邮件，该 API 不会在系统中自动创建用户对象。
- 通过此 API 更新的评论仍可以在需要时进行垃圾邮件检查。
- 通过自定义规则管理页面配置的设置（例如最大评论长度）也会在此适用。
- 要允许用户更新评论文本，只需在请求体中指定 `comment`。我们将生成相应的 `commentHTML`。
  - 如果同时定义了 `comment` 和 `commentHTML`，我们将不会自动生成 HTML。
  - 如果用户在新文本中添加了提及或标签（hashtags），仍会像 `POST` API 那样进行处理。
- 在更新评论的 `commenterEmail` 时，最好同时指定 `userId`。否则，您必须确保该电子邮件对应的用户属于您的租户，否则请求将失败。  


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
	/** 执行更新的用户。如果需要，可用于检查他们是否可以编辑该评论。 **/
    contextUserId?: string
	/** 是否应检查新的评论是否看起来像垃圾邮件？ **/
    doSpamCheck?: 'true' | 'false'
	/** 评论是否应对查看具有相同 urlId 的评论小部件实例的用户“实时”显示。注意：会将学分成本从 1 翻倍为 2。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '评论 PATCH 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]