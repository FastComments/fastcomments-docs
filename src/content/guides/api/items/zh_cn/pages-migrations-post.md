[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

此路由将评论和页面从一个 `urlId` 或域名迁移到另一个，功能与仪表板中的 [Domain Migration tool](/guide-migrations.html#migrating-domains) 相同。每个请求最多可发送 100 个迁移。它们按给定顺序执行，因此先将 `a` 迁移到 `b`，再将 `b` 迁移到 `c` 是可行的。

每个迁移都有一个 `type`：

- `urlId` 将 `urlId` 完全等于 `from` 的页面迁移。`post-1` 不匹配 `post-10`。
- `domain` 在每个 `urlId` 和 `url` 中查找/替换 `from`，并更新每条评论的 `domain`。两个值都必须是域名，例如 `old.example.com`。

评论、页面、页面反应、投票、通知和订阅都会一起迁移。如果目标 `to` 已存在页面，则两个页面将合并。

迁移作为批处理作业运行。响应中包含 `batchJobId`。轮询 [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) 以跟踪其进度。作业完成后会为每个迁移提供结果，并返回一个 `undoId`，您可以在 90 天内将其传递给 [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post)。

请求在排队之前会进行验证。如果任何迁移无效，整个请求将以 `422` 失败，且不会执行任何操作。

排队迁移不收取费用。作业完成后，每迁移 `1000` 条评论收取 `1` 积分，最低收取 `1` 积分。

[inline-code-attrs-start title = '页面迁移 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = '页面迁移请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** 按给定顺序执行。最多 100 条。 **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** 最多 1000 个字符。会去除前后空白。 **/
    from: string
    /** 必须不同于 “from”。最多 1000 个字符。 **/
    to: string
    /** “urlId” 仅匹配完全相同的 urlId。 “domain” 在每个 urlId 和 url 中查找/替换该值。 **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = '页面迁移响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** 失败时包含。对于 invalid-migration，此字段指示第一个无效迁移的索引。 **/
    reason?: string
    /** 传递给 GET /api/v1/batch-jobs/:id 以获取进度和结果。 **/
    batchJobId?: string
}
[inline-code-end]

#### 注意

- 同一租户的作业一次只运行一个，按最旧的先执行。第二个迁移会等待第一个完成。
- 使用 `PATCH /api/v1/pages/:id` 并提供新的 `urlId` 只会更改页面本身，而不会更改其评论。使用此路由来迁移页面的评论。