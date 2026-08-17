[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 用於取得供使用者顯示的評論。例如，它會自動過濾未批准或垃圾評論。

### Pagination

分頁可以依照效能需求與使用情境以兩種方式進行：

1. **最快：Precalculated Pagination**：
   1. 這是使用我們預建小工具與客戶端時 FastComments 的運作方式。
   2. 點擊「next」只會增加頁數。
   3. 你可以將其視為由鍵值儲存庫取得。
   4. 以此方式，只需定義從 `0` 開始的 `page` 參數以及排序方向 `direction`。
   5. 可透過自訂規則調整每頁大小。
2. **最彈性：Flexible Pagination**：
   1. 以此方式你可以自訂 `limit` 與 `skip` 參數。不要傳遞 `page`。
   2. 也支援排序 `direction`。
   3. `limit` 為在套用 `skip` 後要返回的總數量。
      - 例子：當 `page size = 100` 且 `page = 2` 時，設定 `skip = 200, limit = 100`。
   4. 子評論仍會計入分頁。你可以使用 `asTree` 選項繞過此限制。
      - 你可以透過 `limitChildren` 與 `skipChildren` 來分頁子項目。
      - 你可以使用 `maxTreeDepth` 限制返回的討論串深度。

### Threads

1. 使用 `Precalculated Pagination` 時，評論會依 *page* 分組，討論串中的評論會影響整體頁面。
   1. 以此方式，討論串可根據 `parentId` 在客戶端判斷。
   2. 例如，若一頁只有一則頂層評論且有 29 則回覆，且在 API 中設定 `page=0`，你將只取得該頂層評論與 29 個子評論。
2. 使用 `Flexible Pagination` 時，你可以定義 `parentId` 參數。
   1. 設為 `null` 只取得頂層評論。
   2. 然後若要檢視討論串，重新呼叫 API 並傳遞 `parentId`。
   3. 常見做法是先取得頂層評論，接著平行呼叫 API 取得每則評論的子評論。
3. __NEW As of Feb 2023!__ 使用 `&asTree=true` 以樹狀方式取得。
   1. 你可以將其視為 `Flexible Pagination as a Tree`。
   2. 只有頂層評論會計入分頁。
   3. 設定 `parentId=null` 以在根節點開始樹（必須設定 `parentId`）。
   4. 設定 `skip` 與 `limit` 進行分頁。
   5. 設定 `asTree` 為 `true`。
   6. 其信用點成本會增加 `2x`，因為後端必須在此情境下執行更多工作。
   7. 可依需求設定 `maxTreeDepth`、`limitChildren` 與 `skipChildren`。

### Trees Explained

使用 `asTree` 時，分頁的推理可能較為困難。以下是一張實用圖示：

<div class="screenshot white-bg">
    <div class="title">樹狀分頁圖示</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="樹狀分頁圖示" />
</div>

### Fetching Comments in The Context of a User

`/comments` API 可在兩種情境下使用，以因應不同需求：

- 用於返回已排序且帶有資訊標籤的評論，以建構你自己的客戶端。
  - 在此情況下，定義 `contextUserId` 查詢參數。
- 用於從你的後端取得評論，以進行自訂整合。
  - 平台在沒有 `contextUserId` 時會預設使用此方式。

[inline-code-attrs-start title = '預先計算分頁的評論'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '彈性分頁的評論'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '使用者情境下的彈性分頁評論'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '使用者情境下僅頂層評論的彈性分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

可以將返回的評論以樹狀結構取得，且分頁僅計算頂層評論。

[inline-code-attrs-start title = '使用者情境下的樹狀評論'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

想只取得頂層評論與其直接子項目嗎？以下是一種做法：

[inline-code-attrs-start title = '具有最大深度的樹狀評論'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

然而，在你的 UI 中可能需要判斷是否在每則評論上顯示「顯示回覆」按鈕。透過樹狀方式取得評論時，若適用，評論會帶有 `hasChildren` 屬性。

### Get Comments as a Tree, Searching by Hash Tag

可以使用 API 依標籤搜尋，遍及整個租戶（不限於單一頁面或 `urlId`）。

在此範例中，我們省略 `urlId`，並以多個標籤搜尋。API 只會返回同時具備所有請求標籤的評論。

[inline-code-attrs-start title = '使用者情境下依標籤的樹狀評論'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = '評論請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### The Response

[inline-code-attrs-start title = '評論回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Helpful Tips

#### URL ID

你可能想使用帶有 `urlId` 參數的 `Comment` API。你可以先呼叫 `Pages` API，查看可用的 `urlId` 值長什麼樣子。

#### Anonymous Actions

對於匿名評論，你可能想在取得評論以及執行檢舉與封鎖時傳遞 `anonUserId`。

(!) 這在許多應用商店是必須的，因為使用者必須能檢舉他們能看到的使用者產生內容，即使未登入。未執行此步驟可能導致你的應用被移除出該商店。

#### Comments Not Being Returned

確認你的評論已被批准，且不是垃圾評論。

---