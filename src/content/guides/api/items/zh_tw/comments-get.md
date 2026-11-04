[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 用於取得要顯示給使用者的留言。例如，它會自動過濾掉未核准或垃圾留言。

### 分頁

分頁可以透過以下兩種方式之一完成，取決於效能需求與使用情境：

1. 最快：**預先計算分頁**：
   1. 當您使用我們的預建小工具與用戶端時，FastComments 就是這樣運作的。
   2. 點擊「下一頁」只會增加頁數。
   3. 您可以將此視為透過鍵值儲存來擷取。
   4. 如此一來，只需定義一個從 `0` 開始的 `page` 參數，以及作為排序方向的 `direction`。
   5. 頁面大小可透過自訂規則進行自訂。
2. 最有彈性：**彈性分頁**：
   1. 如此一來，您可以定義自訂的 `limit` 與 `skip` 參數。請勿傳遞 `page`。
   2. 也支援排序 `direction`。
   3. `limit` 是套用 `skip` 之後要回傳的總數。
      - 範例：當 `page size = 100` 且 `page = 2` 時，設定 `skip = 200, limit = 100`。
   4. 子留言仍會計入分頁。您可以使用 `asTree` 選項來解決這個問題。
      - 您可以透過 `limitChildren` 與 `skipChildren` 來分頁子留言。
      - 您可以透過 `maxTreeDepth` 限制回傳討論串的深度。

### 討論串

1. 使用 `Precalculated Pagination` 時，留言會依 *頁面* 分組，且討論串中的留言會影響整體頁面。
   1. 如此一來，用戶端可以根據 `parentId` 來判斷討論串。
   2. 例如，一個頁面有一則頂層留言與 29 則回覆，並在 API 中設定 `page=0` — 您將只會取得該頂層留言與 29 則子留言。
2. 使用 `Flexible Pagination` 時，您可以定義 `parentId` 參數。
   1. 將其設為 null 即可只取得頂層留言。
   2. 然後若要檢視討論串，請再次呼叫 API 並傳遞 `parentId`。
   3. 常見的做法是先呼叫 API 取得頂層留言，然後平行呼叫 API 以取得每則留言子留言的內容。
3. __2023 年 2 月起新功能！__ 使用 `&asTree=true` 以樹狀結構擷取。
   1. 您可以將此視為 `Flexible Pagination as a Tree`。
   2. 只有頂層留言會計入分頁。
   3. 設定 `parentId=null` 以從根節點開始樹狀結構（您必須設定 `parentId`）。
   4. 設定 `skip` 與 `limit` 進行分頁。
   5. 將 `asTree` 設為 `true`。
   6. 點數成本會增加 `2x`，因為在此情境下我們的後端需要做更多工作。
   7. 依需求設定 `maxTreeDepth`、`limitChildren` 與 `skipChildren`。

### 樹狀結構說明

使用 `asTree` 時，分頁可能不容易理解。以下是方便的圖示：

<div class="screenshot white-bg">
    <div class="title">樹狀分頁圖解</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="樹狀分頁圖解" />
</div>

### 在使用者情境中擷取留言

`/comments` API 可在兩種情境中使用，適用於不同的使用案例：

- 用於回傳已排序並標記資訊的留言，以建立您自己的用戶端。
  - 在此情況下，請定義 `contextUserId` 查詢參數。
- 用於從您的後端擷取留言以進行自訂整合。
  - 若未提供 `contextUserId`，平台將預設為此模式。

[inline-code-attrs-start title = '留言預先計算分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = '留言彈性分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = '使用者情境中的留言彈性分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = '使用者情境中僅限頂層留言的彈性分頁'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### 以樹狀結構取得留言

可以讓留言以樹狀結構回傳，且分頁只計算頂層留言。

[inline-code-attrs-start title = '使用者情境中的樹狀留言'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

只想取得頂層留言與直接子留言？以下是一種方法：

[inline-code-attrs-start title = '具有最大深度的樹狀留言'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

不過，在您的 UI 中，您可能需要知道是否要在每則留言上顯示「顯示回覆」按鈕。以樹狀結構擷取留言時，在適用的情況下，留言上會標記 `hasChildren` 屬性。

### 以樹狀結構取得留言，並依主題標籤搜尋

可以使用 API 依主題標籤搜尋，範圍涵蓋您的整個租戶（不限於單一頁面或 `urlId`）。

在此範例中，我們省略 `urlId`，並以多個主題標籤進行搜尋。API 只會回傳具有所有要求主題標籤的留言。

[inline-code-attrs-start title = '使用者情境中依主題標籤的樹狀留言'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** 留言所關聯的 urlId（頁面網址或文章 ID）。 **/
    urlId?: string
    /** 限制只回傳此使用者的留言。 **/
    userId?: string
    /** 使用此參數依主題標籤搜尋。若要深入篩選多個主題標籤的交集，請使用 &hashTag=a&hashTag=b。 **/
    hashTag?: string
    /** 排序方向。預設為 MR（最相關）。其他選項為 OF（最舊優先）與 NF（最新優先）。 **/
    direction?: 'MR' | 'OF' | 'NF'
    /** 預先計算分頁：要擷取的頁面，從 0 開始。傳遞 -1 以取得所有留言（最多 250 則）。 **/
    page?: number
    /** 彈性分頁：我們應回傳多少則留言？ **/
    limit?: number
    /** 彈性分頁：每個父留言應回傳多少則子留言？ **/
    limitChildren?: number
    /** 彈性分頁：我們應略過多少則留言？ **/
    skip?: number
    /** 彈性分頁：每個父留言應略過多少則子留言？ **/
    skipChildren?: number
    /** 用於判斷被封鎖與被檢舉的留言。 **/
    contextUserId?: string
    /** 用於判斷被封鎖與被檢舉的留言。 **/
    anonUserId?: string
    ///** 用於擷取子留言。 **/
    parentId?: string
    /** 用於以樹狀結構擷取。 **/
    asTree?: boolean
    /** 應回傳樹狀結構多深的資料？0 不回傳子留言。1 回傳直接子留言，依此類推。 **/
    maxTreeDepth?: number
}
[inline-code-end]

### 回應

[inline-code-attrs-start title = '留言回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** 失敗時包含。 **/
    reason?: string
    /** 留言！ **/
    comments: Comment[]
}
[inline-code-end]

### 實用提示

#### URL ID

您可能會想搭配 `urlId` 參數使用 `Comment` API。您可以先呼叫 `Pages` API，以查看您可用的 `urlId` 值長什麼樣子。

#### 匿名操作

對於匿名留言，您在擷取留言以及執行檢舉與封鎖時，可能需要傳遞 `anonUserId`。

(!) 許多應用程式商店都要求此參數，因為使用者必須能夠檢舉他們看得到的使用者建立的內容，即使他們未登入也一樣。若不這樣做，可能會導致您的應用程式從該商店被移除。

#### 留言未被回傳

請檢查您的留言是否已核准，且不是垃圾留言。