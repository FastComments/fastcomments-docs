[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

此路由將評論和頁面從一個 `urlId` 或網域移動到另一個，功能與儀表板中的 [Domain Migration tool](/guide-migrations.html#migrating-domains) 相同。每個請求最多可傳送 100 個遷移。它們會依給定順序執行，因此 `a` 到 `b` 再到 `b` 到 `c` 是可行的。

每個遷移都有一個 `type`：

- `urlId` 會移動 `urlId` 完全等於 `from` 的頁面。`post-1` 不會匹配 `post-10`。
- `domain` 會在每個 `urlId` 和 `url` 中尋找/取代 `from`，並更新每則評論的 `domain`。兩個值都必須是網域，例如 `old.example.com`。

評論、頁面、頁面回應、投票、通知與訂閱都會一起移動。如果 `to` 位置已存在頁面，兩個頁面將合併。

遷移會作為批次工作執行。回應中包含 `batchJobId`。請輪詢 [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) 以追蹤其進度。工作完成後會針對每個遷移提供結果，並會產生一個 `undoId`，您可在 90 天內將其傳遞給 [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) 以復原。

請求會在排入佇列前先進行驗證。如果有任何遷移無效，整個請求會以 `422` 錯誤失敗，且不會執行任何操作。

排入遷移佇列不會產生費用。工作完成後，依每遷移 `1000` 條評論收取 `1` 點信用，最低收取 `1` 點。

[inline-code-attrs-start title = '頁面遷移 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '頁面遷移 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** 按給定順序執行。最多 100 個。 **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** 最多 1000 個字元。會移除前後空白。 **/
    from: string
    /** 必須與 "from" 不同。最多 1000 個字元。 **/
    to: string
    /** "urlId" 只匹配完全相同的 urlId。 "domain" 會在每個 urlId 和 url 中尋找/取代該值。 **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = '頁面遷移 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** 失敗時包含此欄位。對於 invalid-migration，此欄位指示第一個無效遷移的索引。 **/
    reason?: string
    /** 用於 GET /api/v1/batch-jobs/:id 以取得進度和結果。 **/
    batchJobId?: string
}
[inline-code-end]

#### 注意事項

- 同一租戶的工作一次只會執行一個，依最舊的先執行。第二個遷移會等待第一個完成。
- 使用 `PATCH /api/v1/pages/:id` 並提供新的 `urlId` 只會變更頁面本身，不會影響其評論。請使用此路由來移動頁面的評論。

---