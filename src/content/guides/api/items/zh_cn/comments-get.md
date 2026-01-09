[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 用于获取要向用户显示的评论。例如，它会自动过滤掉未批准或垃圾评论。

### 分页

分页可以根据性能需求和用例通过两种方式之一完成：

1. 最快：**预计算分页（Precalculated Pagination）**：
   1. 当您使用我们预构建的小部件和客户端时，这就是 FastComments 的工作方式。
   2. 单击“下一页”仅会增加页数。
   3. 您可以将其视为通过键值存储检索。
   4. 用这种方式，只需定义从 `0` 开始的 `page` 参数和作为 `direction` 的排序方向。
   5. 页面大小可以通过自定义规则进行调整。
2. 最灵活：**灵活分页（Flexible Pagination）**：
   1. 在这种方式中，您可以定义自定义的 `limit` 和 `skip` 参数。不要传递 `page`。
   2. 也支持排序 `direction`。
   3. `limit` 是在应用了 `skip` 后要返回的总数量。
      - 示例：当页面大小 = 100 且 `page = 2` 时，设置 `skip = 200, limit = 100`。
   4. 子评论仍计入分页。您可以使用 `asTree` 选项绕过此问题。
      - 您可以通过 `limitChildren` 和 `skipChildren` 对子评论进行分页。
      - 您可以通过 `maxTreeDepth` 限制返回的线程深度。

### 线程（Threads）

1. 使用 `Precalculated Pagination` 时，评论按*页面*分组，线程中的评论会影响整个页面。
   1. 这样，线程可以基于客户端的 `parentId` 来确定。
   2. 例如，在一个页面中有一个顶级评论和 29 个回复，并在 API 中设置 `page=0` —— 您将只获得顶级评论和这 29 个子评论。
   3. [示例图，说明多页。](https://blog.winricklabs.com/images/fc-pagination02.png)
2. 使用 `Flexible Pagination` 时，您可以定义 `parentId` 参数。
   1. 将其设置为 null 以仅获取顶级评论。
   2. 然后要查看线程，请再次调用 API 并传递 `parentId`。
   3. 一个常见的解决方案是对顶级评论进行一次 API 调用，然后并行调用 API 以获取每个评论的子评论。
3. __新功能（截至 2023 年 2 月）!__ 使用 `&asTree=true` 作为树来获取。
   1. 您可以将其视为“作为树的灵活分页（Flexible Pagination as a Tree）”。
   2. 只有顶级评论计入分页。
   3. 将 `parentId=null` 设置为从根开始树（您必须设置 `parentId`）。
   4. 设置 `skip` 和 `limit` 以进行分页。
   5. 将 `asTree` 设置为 `true`。
   6. 信用点成本增加 `2x`，因为在这种情况下我们的后端需要做更多工作。
   7. 根据需要设置 `maxTreeDepth`、`limitChildren` 和 `skipChildren`。

### 树说明

在使用 `asTree` 时，理解分页可能会很困难。这里有一张便捷的图示：

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### 在用户上下文中获取评论

`/comments` API 可以在两种上下文中使用，以满足不同的用例：

- 用于返回已排序并标注有用于构建您自己的客户端的信息的评论。
  - 在这种情况下，定义一个 `contextUserId` 查询参数。
- 用于从您的后端获取评论以进行自定义集成。
  - 如果不提供 `contextUserId`，平台将默认采用此方式。

[inline-code-attrs-start title = '评论（预计算分页）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '评论（灵活分页）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '用户上下文中的评论（灵活分页）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '仅顶级评论的用户上下文中的评论（灵活分页）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### 以树的形式获取评论

可以将评论作为树返回，分页只计算顶级评论。

[inline-code-attrs-start title = '用户上下文中的树状评论'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

想只获取顶级评论和其直接子评论？这是一种方法：

[inline-code-attrs-start title = '树状评论（最大深度）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

但是，在您的 UI 中，您可能需要知道是否在每条评论上显示“显示回复”按钮。通过树获取评论时，当适用时，评论上会带上 `hasChildren` 属性。

### 以树的形式获取评论，按标签搜索

可以使用 API 按标签搜索整个租户的评论（不局限于某个页面或 `urlId`）。

在此示例中，我们省略了 `urlId`，并按多个标签进行搜索。API 只会返回包含所有请求标签的评论。

[inline-code-attrs-start title = '用户上下文中的树状评论（按标签）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### 响应

[inline-code-attrs-start title = '评论响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### 有用的小贴士

#### URL ID

您可能希望在使用 `Comment` API 时使用 `urlId` 参数。您可以先调用 `Pages` API，以查看可用的 `urlId` 值的样子。

#### 匿名操作

对于匿名评论，您可能希望在获取评论以及执行标记和屏蔽操作时传递 `anonUserId`。

(!) 这对于许多应用商店是必需的，因为用户必须能够对他们能看到的用户生成内容进行标记，即使他们未登录。不这样做可能会导致您的应用被该商店移除。

#### 评论未返回

检查您的评论是否已批准，且不是垃圾评论。

---