[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 用于获取评论以显示给用户。例如，它会自动过滤未批准或垃圾评论。

### Pagination

分页可以通过以下两种方式之一完成，具体取决于性能需求和使用场景：

1. **最快：** **Precalculated Pagination**：
   1. 这就是在使用我们预构建的小部件和客户端时 FastComments 的工作方式。
   2. 点击“下一页”只会增加页码。
   3. 可以将其视为通过键值存储检索。
   4. 在这种方式下，只需定义从 `0` 开始的 `page` 参数以及排序方向 `direction`。
   5. 页面大小可以通过自定义规则进行定制。
2. **最灵活：** **Flexible Pagination**：
   1. 在这种方式下，您可以定义自定义的 `limit` 和 `skip` 参数。不要传递 `page`。
   2. 也支持排序 `direction`。
   3. `limit` 是在应用 `skip` 后返回的总数量。
      - 示例：当 `page size = 100` 且 `page = 2` 时，设置 `skip = 200, limit = 100`。
   4. 子评论仍然计入分页。您可以使用 `asTree` 选项来规避此问题。
      - 您可以通过 `limitChildren` 和 `skipChildren` 对子评论进行分页。
      - 您可以通过 `maxTreeDepth` 限制返回线程的深度。

### Threads

1. 使用 `Precalculated Pagination` 时，评论按 *page* 分组，线程中的评论会影响整体页面。
   1. 在这种方式下，线程可以基于 `parentId` 在客户端确定。
   2. 例如，页面上有一个顶级评论和 29 条回复，并在 API 中设置 `page=0`——您将只得到顶级评论及其 29 条子评论。
2. 使用 `Flexible Pagination` 时，您可以定义 `parentId` 参数。
   1. 将其设为 null 只获取顶级评论。
   2. 然后要查看线程，重新调用 API 并传递 `parentId`。
   3. 常见的解决方案是先为顶级评论进行一次 API 调用，然后并行调用 API 获取每条评论的子评论。
3. __NEW 自 2023 年 2 月起！__ 使用 `&asTree=true` 以树形方式获取。
   1. 您可以将其视为 `Flexible Pagination as a Tree`（树形灵活分页）。
   2. 仅顶级评论计入分页。
   3. 将 `parentId=null` 设置为从根开始树（必须设置 `parentId`）。
   4. 使用 `skip` 和 `limit` 进行分页。
   5. 将 `asTree` 设置为 `true`。
   6. 积分消耗增加 `2x`，因为后端在此场景下需要做更多工作。
   7. 根据需要设置 `maxTreeDepth`、`limitChildren` 和 `skipChildren`。

### Trees Explained

使用 `asTree` 时，分页的推理可能比较困难。以下是一张实用的示意图：

<div class="screenshot white-bg">
    <div class="title">树形分页示意图</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="树形分页示意图" />
</div>

### Fetching Comments in The Context of a User

`/comments` API 可在两种上下文中使用，以满足不同的使用场景：

- 用于返回已排序并带有信息标签的评论，以便构建您自己的客户端。
  - 在这种情况下，定义 `contextUserId` 查询参数。
- 用于从您的后端获取评论，以进行自定义集成。
  - 平台将在没有 `contextUserId` 的情况下默认使用此方式。

[inline-code-attrs-start title = '评论 预计算 分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '评论 灵活 分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '用户上下文中的评论 灵活分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '用户上下文中仅顶级评论的灵活分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

可以将返回的评论以树形结构获取，分页仅计入顶级评论。

[inline-code-attrs-start title = '用户上下文中的评论树形结构'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

想只获取顶级评论及其直接子评论吗？下面是一种方式：

[inline-code-attrs-start title = '具有最大深度的评论树形结构'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

然而，在您的 UI 中可能需要知道是否在每条评论上显示“显示回复”按钮。通过树形获取评论时，会在适用的评论上标记 `hasChildren` 属性。

### Get Comments as a Tree, Searching by Hash Tag

可以使用 API 按标签搜索，跨整个租户（不限于单页或 `urlId`）。

在此示例中，我们省略了 `urlId`，并按多个标签搜索。API 只会返回同时包含所有请求标签的评论。

[inline-code-attrs-start title = '用户上下文中按标签的评论树形结构'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = '评论请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 与评论关联的 urlId（页面 URL 或文章 ID）。 **/
    urlId?: string
    /** 限制此用户返回的评论。 **/
    userId?: string
    /** 使用此项按标签搜索。要深入多个标签的交集，请使用 &hashTag=a&hashTag=b。 **/
    hashTag?: string
    /** 排序方向。默认是 MR（最相关）。其他选项有 OF（最旧优先）和 NF（最新优先）。 **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 预计算分页：要获取的页码，从 0 开始。传 -1 可获取所有评论（最多 250 条）。 **/
    page?: number
    /** 灵活分页：我们应返回多少条评论？ **/
    limit?: number
    /** 灵活分页：每个父级应返回多少条子评论？ **/
    limitChildren?: number
    /** 灵活分页：应跳过多少条评论？ **/
    skip?: number
    /** 灵活分页：每个父级应跳过多少条子评论？ **/
    skipChildren?: number
    /** 用于确定被阻止和被标记的评论。 **/
    contextUserId?: string
    /** 用于确定被阻止和被标记的评论。 **/
    anonUserId?: string
    /** 用于获取子评论。 **/
    parentId?: string
    /** 用于以树形方式获取。 **/
    asTree?: boolean
    /** 应返回树的多深数据？0 表示不返回子评论，1 表示返回直接子评论，依此类推。 **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = '评论响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** 失败时包含。 **/
    reason?: string
    /** 评论！ **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

您可能想在使用 `Comment` API 时提供 `urlId` 参数。您可以先调用 `Pages` API，查看可用的 `urlId` 值是什么样的。

#### Anonymous Actions

对于匿名评论，您可能需要在获取评论以及进行标记和阻止时传递 `anonUserId`。

(!) 这在许多应用商店是必需的，因为用户必须能够标记他们看到的用户生成内容，即使他们未登录。未执行此操作可能导致您的应用被从该商店下架。

#### Comments Not Being Returned

检查您的评论是否已批准且不是垃圾评论。

---