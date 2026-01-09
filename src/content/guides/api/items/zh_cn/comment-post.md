[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 端点提供创建评论的功能。

常见用例包括自定义 UI、集成或导入。

注意：

- 如果需要，此 API 可以将评论小部件实时更新（这会将 `creditsCost` 从 `1` 增加到 `2`）。
- 如果提供了电子邮件地址，此 API 会在我们的系统中自动创建用户对象。
- 尝试保存两个电子邮件不同但用户名相同的评论，会导致第二条评论出现错误。 
- 如果您指定了 `parentId`，并且子评论的 `notificationSentForParent` 为 false，**我们会为父评论发送通知**。此操作每小时进行一次（我们会将通知合并以减少发送的邮件数量）。
- 如果您希望在创建用户时发送欢迎邮件，或发送评论验证邮件，请在查询参数中将 `sendEmails` 设置为 `true`。
- 通过此 API 创建的评论会显示在管理应用的分析 (Analytics) 和审核 (Moderation) 页面中。
- 如果启用了该设置，评论者名称和评论文本中的“脏词”仍会被屏蔽。
- 通过此 API 创建的评论仍然可以（如需）进行垃圾评论检查。
- 通过自定义规则管理页面配置的设置（例如最大评论长度）也将适用于此处。

要在评论小部件中显示，提交的最低数据如下：

[inline-code-attrs-start title = '最小评论 POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

更现实的请求可能如下所示：

[inline-code-attrs-start title = '评论 POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = '评论 POST 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** 指示该评论是否应对查看相同 urlId 的评论小部件实例的用户“实时”显示。注意：会将信用点消耗从 1 翻倍为 2。 **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '评论 POST 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** 失败时包含。 **/
    reason?: string
    /** 创建的评论。 **/
    comment?: Comment
    /** 关联的用户，可能已存在也可能是新创建的。 **/
    user?: User
}
[inline-code-end]