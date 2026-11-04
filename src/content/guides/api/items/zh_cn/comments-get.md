[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 用于获取要展示给用户的评论。例如，它会自动过滤掉未通过审核或被标记为垃圾评论的内容。

### 分页

根据性能需求和用例，可以通过以下两种方式之一进行分页：

1. 最快：**预计算分页**：
   1. 当您使用我们的预构建组件和客户端时，FastComments 就是以这种方式工作的。
   2. 点击“下一页”只需增加页数。
   3. 您可以将其想象为通过键值存储进行检索。
   4. 通过这种方式，只需定义一个从 `0` 开始的 `page` 参数，并将排序方向定义为 `direction`。
   5. 页面大小可以通过自定义规则进行配置。
2. 最灵活：**灵活分页**：
   1. 通过这种方式，您可以定义自定义的 `limit` 和 `skip` 参数。不要传递 `page`。
   2. 也支持排序 `direction`。
   3. `limit` 是应用 `skip` 之后要返回的总数。
      - 示例：当 `page size = 100` 且 `page = 2` 时，设置 `skip = 200, limit = 100`。
   4. 子评论仍计入分页。您可以使用 `asTree` 选项来规避此问题。
      - 您可以通过 `limitChildren` 和 `skipChildren` 对子评论进行分页。
      - 您可以通过 `maxTreeDepth` 限制返回线程的深度。

### 线程

1. 使用 `Precalculated Pagination` 时，评论按*页面*分组，线程中的评论会影响整个页面。
   1. 通过这种方式，客户端可以根据 `parentId` 确定线程。
   2. 例如，对于一个包含一条顶级评论和 29 条回复的页面，在 API 中设置 `page=0` 后——您将只获得该顶级评论及其 29 条子评论。
2. 使用 `Flexible Pagination` 时，您可以定义 `parentId` 参数。
   1. 将其设置为 null 以仅获取顶级评论。
   2. 然后要查看线程，请再次调用 API 并传递 `parentId`。
   3. 一种常见的解决方案是先调用 API 获取顶级评论，然后并行调用 API 获取每条评论的子评论。
3. __2023 年 2 月新增！__ 使用 `&asTree=true` 以树的形式获取。
   1. 您可以将其想象为 `Flexible Pagination as a Tree`。
   2. 分页中只计算顶级评论。
   3. 设置 `parentId=null` 以从根节点开始树（您必须设置 `parentId`）。
   4. 设置 `skip` 和 `limit` 进行分页。
   5. 将 `asTree` 设置为 `true`。
   6. 积分消耗增加 `2x`，因为在此场景下我们的后端需要做更多的工作。
   7. 根据需要设置 `maxTreeDepth`、`limitChildren` 和 `skipChildren`。

### 树结构说明

使用 `asTree` 时，分页可能难以理解。这里有一张实用的图示：

<div class="screenshot white-bg">
    <div class="title">树分页图示</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="树分页图示" />
</div>

### 在用户上下文中获取评论

`/comments` API 可以在两种上下文中使用，适用于不同的用例：

- 用于返回已排序并带有信息的评论，以便构建您自己的客户端。
  - 在这种情况下，请定义 `contextUserId` 查询参数。
- 用于从您的后端获取评论以进行自定义集成。
  - 在没有 `contextUserId` 的情况下，平台将默认使用此方式。

[inline-code-attrs-start title = '评论预计算分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '评论灵活分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '用户上下文中的评论灵活分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '用户上下文中仅获取顶级评论的灵活分页'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### 以树的形式获取评论

可以将评论以树的形式返回，分页只计算顶级评论。

[inline-code-attrs-start title = '用户上下文中以树形式获取评论'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

只想获取顶级评论及其直接子评论？这里有一种方法：

[inline-code-attrs-start title = '带最大深度的树形式评论'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

但是，在您的 UI 中，您可能需要知道是否应在每条评论上显示“显示回复”按钮。通过树的形式获取评论时，在适用的情况下，评论上会附加一个 `hasChildren` 属性。

### 以树的形式获取评论，按话题标签搜索

可以通过 API 按话题标签进行搜索，覆盖您的整个租户（不限于单个页面或 `urlId`）。

在此示例中，我们省略了 `urlId`，并按多个话题标签进行搜索。API 将只返回包含所有请求话题标签的评论。

[inline-code-attrs-start title = '用户上下文中按话题标签以树形式获取评论'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### 所有请求参数

[inline-code-attrs-start title = '评论请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 评论所关联的 urlId（页面 URL 或文章 ID）。 **/
    urlId?: string
    /** 限制返回该用户的评论。 **/
    userId?: string
    /** 用于按话题标签搜索。要深入到多个话题标签的交集，请使用 &hashTag=a&hashTag=b。 **/
    hashTag?: string
    /** 排序方向。默认为 MR（最相关）。其他选项为 OF（最旧优先）和 NF（最新优先）。 **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 预计算分页：要获取的页面，从 0 开始。传递 -1 获取所有评论（最多 250 条）。 **/
    page?: number
    /** 灵活分页：应返回多少条评论？ **/
    limit?: number
    /** 灵活分页：每个父评论应返回多少条子评论？ **/
    limitChildren?: number
    /** 灵活分页：应跳过多少条评论？ **/
    skip?: number
    /** 灵活分页：每个父评论应跳过多少条子评论？ **/
    skipChildren?: number
    /** 用于确定被屏蔽和被标记的评论。 **/
    contextUserId?: string
    /** 用于确定被屏蔽和被标记的评论。 **/
    anonUserId?: string
    /** 用于获取子评论。 **/
    parentId?: string
    /** 用于以树的形式获取。 **/
    asTree?: boolean
    /** 应返回树中多深的数据？0 表示不返回子评论。1 表示返回直接子评论，依此类推。 **/
    maxTreeDepth?: number
}
[inline-code-end]

### 响应

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

### 实用提示

#### URL ID

您可能希望使用带有 `urlId` 参数的 `Comment` API。您可以先调用 `Pages` API，查看可用的 `urlId` 值是什么样的。

#### 匿名操作

对于匿名评论，您可能希望在获取评论以及执行标记和屏蔽操作时传递 `anonUserId`。

(!) 许多应用商店都要求这样做，因为用户必须能够标记他们可以看到的用户创建内容，即使他们未登录。不这样做可能导致您的应用从相应商店中被下架。

#### 评论未被返回

请检查您的评论是否已通过审核，且不是垃圾评论。