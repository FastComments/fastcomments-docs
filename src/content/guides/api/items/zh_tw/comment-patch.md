[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端點提供更新單則評論的功能。

Notes:

- 此 API 可在需要時將評論元件「即時」更新（這會將基本 `creditsCost` 從 `1` 提高為 `2`）。
  - 這可讓在頁面之間遷移評論成為「即時」（更改 `urlId`）。
  - 由於頁面會被預先計算且此操作耗費大量 CPU，遷移會額外收取 `2` 點額度。
- 與建立 API 不同，如果提供 email，本 API 不會自動在系統中建立使用者物件。
- 透過此 API 更新的評論仍可在需要時進行垃圾郵件檢查。
- 若透過自訂規則管理頁面設定的配置（例如最大評論長度），將在此生效。
- 若要允許使用者更新他們的評論文字，只需在請求主體中指定 `comment`。我們會生成相應的 `commentHTML`。
  - 若同時定義了 `comment` 與 `commentHTML`，我們將不會自動生成 HTML。
  - 若使用者在新文字中加入提及或標籤，依然會像 `POST` API 一樣處理。
- 在更新評論的 `commenterEmail` 時，最好也指定 `userId`。否則，您必須確保該 email 所對應的使用者屬於您的租戶，否則請求將失敗。  
- 如果目標評論被鎖定（`isLocked: true`），則請求會以 `code: 'locked'` 被拒絕。請先解除鎖定再更新，若需要可於更新後重新鎖定。


[inline-code-attrs-start title = '最小評論 PATCH cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = '評論 PATCH 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** 執行更新的使用者。若需要，可用來檢查他們是否能編輯該評論。  **/
    contextUserId?: string
	/** 我們是否要檢查新的評論是否看起來像垃圾郵件？  **/
    doSpamCheck?: 'true' | 'false'
	/** 該評論是否應對觀看具有相同 urlId 的評論元件實例的使用者顯示為「即時」。注意：會將點數成本從 1 翻倍為 2。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '評論 PATCH 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** 失敗時會包含。 **/
    reason?: string
}
[inline-code-end]