[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 用於取得要顯示給使用者的留言。例如，它會自動過濾未核准或垃圾留言。

### 分頁

分頁可以用兩種方式之一，取決於效能需求和使用情境：

1. Fastest: **Precalculated Pagination**:
   1. 這是當您使用我們預建的小工具與客戶端時 FastComments 的運作方式。
   2. 點擊「下一頁」只是簡單地增加頁數。
   3. 你可以把這視為由鍵值儲存取得。
   4. 因此，只需定義從 `0` 開始的 `page` 參數和一個作為 `direction` 的排序方向。
   5. 頁面大小可透過自訂規則進行調整。
2. Most Flexible: **Flexible Pagination**:
   1. 在這種方式中你可以定義自訂的 `limit` 和 `skip` 參數。不要傳遞 `page`。
   2. 也支援排序 `direction`。
   3. `limit` 是在套用 `skip` 後要回傳的總數。
      - 例子：當 `page size = 100` 且 `page = 2` 時，設定 `skip = 200, limit = 100`。
   4. 子留言仍會被計入分頁。你可以使用 `asTree` 選項來避開這點。
      - 你可以透過 `limitChildren` 和 `skipChildren` 對子留言進行分頁。
      - 你可以透過 `maxTreeDepth` 限制回傳的討論串深度。

### 討論串

1. 當使用 `Precalculated Pagination` 時，留言會以「頁」分組，而討論串內的留言會影響整體頁面。
   1. 如此一來，客戶端可根據 `parentId` 判斷討論串。
   2. 例如，某頁有一則頂層留言與 29 則回覆，並在 API 中設定 `page=0` — 你會只得到該頂層留言與 29 個子留言。
   3. [範例圖片，說明多頁情形。](https://blog.winricklabs.com/images/fc-pagination02.png)
2. 當使用 `Flexible Pagination` 時，你可以定義 `parentId` 參數。
   1. 將其設為 null 以僅取得頂層留言。
   2. 然後要查看討論串時，再次呼叫 API 並傳入 `parentId`。
   3. 常見的解法是先對頂層留言進行 API 呼叫，然後並行對每則留言的子留言進行 API 呼叫。
3. __NEW As of Feb 2023!__ 使用 `&asTree=true` 以樹狀方式取得。
   1. 你可以把這視為 `Flexible Pagination as a Tree`。
   2. 分頁僅計算頂層留言。
   3. 將 `parentId=null` 設為樹的根（你必須設定 `parentId`）。
   4. 設定 `skip` 和 `limit` 以進行分頁。
   5. 將 `asTree` 設為 `true`。
   6. 消耗的信用額度會增加為 `2x`，因為後端在此情況下須做更多工作。
   7. 視需要設定 `maxTreeDepth`、`limitChildren` 及 `skipChildren`。

### 樹狀結構說明

當使用 `asTree` 時，理解分頁可能會比較困難。這裡有一張方便的圖示：

<div class="screenshot white-bg">
    <div class="title">樹狀分頁示意圖</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="樹狀分頁示意圖" />
</div>

### 在使用者情境下取得留言

`/comments` API 可用於兩種情境，對應不同的使用案例：

- 用於回傳已排序並標記附加資訊以便建構你自己的客戶端的留言。
  - 在這種情況下，定義一個 `contextUserId` 查詢參數。
- 用於從你的後端擷取留言以做自訂整合。
  - 若未提供 `contextUserId`，平台預設會採用此方式。

[inline-code-attrs-start title = '留言：預先計算分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '留言：彈性分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '留言：使用者情境下的彈性分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '留言：使用者情境下僅頂層留言的彈性分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### 以樹狀取得留言

可以將回傳的留言以樹狀結構呈現，分頁只計算頂層留言。

[inline-code-attrs-start title = '留言：使用者情境下的樹狀取得'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

想只取得頂層留言與其直接子留言？以下是一種方式：

[inline-code-attrs-start title = '留言：樹狀取得（最大深度）'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

不過在你的 UI 中你可能需要知道是否要在每則留言上顯示「顯示回覆」按鈕。當以樹狀方式擷取留言時，適用時會在留言上附加 `hasChildren` 屬性。

### 以樹狀取得留言，依標籤搜尋

可以使用 API 依標籤搜尋整個租戶中的留言（不侷限於單一頁面或 `urlId`）。

在此範例中，我們省略了 `urlId`，並以多個標籤進行搜尋。API 只會回傳具有所有請求標籤的留言。

[inline-code-attrs-start title = '留言：使用者情境下依標籤的樹狀取得'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### 所有請求參數

[inline-code-attrs-start title = '留言請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 留言所關聯的 urlId（頁面 URL 或文章 ID）。 **/
    urlId?: string
    /** 限制由此使用者回傳的留言。 **/
    userId?: string
    /** 用此參數以標籤搜尋。要縮小至多個標籤的交集，使用 &hashTag=a&hashTag=b。 **/
    hashTag?: string
    /** 排序方向。預設為 MR（最相關）。其他選項為 OF（最舊優先）和 NF（最新優先）。 **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 預先計算分頁：要擷取的頁面，從 0 開始。傳入 -1 可取得所有留言（最多 250 筆）。 **/
    page?: number
    /** 彈性分頁：我們應回傳多少則留言？ **/
    limit?: number
    /** 彈性分頁：對每個父留言應回傳多少子留言？ **/
    limitChildren?: number
    /** 彈性分頁：應跳過多少則留言？ **/
    skip?: number
    /** 彈性分頁：對每個父留言應跳過多少子留言？ **/
    skipChildren?: number
    /** 用於判定被封鎖或被檢舉的留言。 **/
    contextUserId?: string
    /** 用於判定被封鎖或被檢舉的留言。 **/
    anonUserId?: string
    /** 用於擷取子留言。 **/
    parentId?: string
    /** 用於以樹狀格式擷取。 **/
    asTree?: boolean
    /** 要在樹狀結構中回傳到多深？0 會不回傳任何子留言。1 會回傳直接子留言，依此類推。 **/
    maxTreeDepth?: number
}
[inline-code-end]

### 回應

[inline-code-attrs-start title = '留言回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** 失敗時會包含。 **/
    reason?: string
    /** 留言！ **/
    comments: Comment[]
}
[inline-code-end]

### 有用的小提示

#### URL ID

你可能會想在呼叫 `Comment` API 時使用 `urlId` 參數。你可以先呼叫 `Pages` API，查看可用的 `urlId` 值長什麼樣子。 

#### 匿名操作

對於匿名留言，你可能會想在取得留言以及進行檢舉和封鎖時傳入 `anonUserId`。

(!) 許多應用商店要求如此，因為使用者必須能夠檢舉他們能看到的使用者產生內容，即使他們沒有登入。不這樣做可能會導致你的應用被該商店移除。

#### 留言未被回傳

請確認你的留言是否已核准，且不是垃圾留言。