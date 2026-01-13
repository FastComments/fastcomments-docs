[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端點提供更新單一留言的功能。

注意事項：

- 如需，此 API 可以使留言小工具「即時」更新（會將基本 `creditsCost` 從 `1` 提高到 `2`）。
  - 這可讓在不同頁面之間遷移留言成為「即時」（變更 `urlId`）。
  - 遷移會額外花費 `2` 點數，因為頁面會被預先計算，且這對 CPU 要求高。
- 與建立 API 不同，若提供電子郵件，本 API 不會在我們系統中自動建立使用者物件。
- 透過本 API 更新的留言仍可在需要時進行垃圾留言檢查。
- 透過自訂規則（Customization Rule）管理頁面設定的配置（例如最大留言長度）也會在此適用。
- 若要允許使用者更新留言內容，只需在請求主體中指定 `comment`。我們將產生對應的 `commentHTML`。
  - 如果同時定義 `comment` 與 `commentHTML`，我們將不會自動生成 HTML。
  - 如果使用者在新文字中加入提及或主題標籤（hashtags），仍會像 `POST` API 一樣進行處理。
- 在更新留言的 `commenterEmail` 時，最好同時指定 `userId`。否則，您必須確保該電子郵件對應的使用者屬於您的租戶，否則請求將失敗。  


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
	/** 執行更新的使用者。若需要可用來檢查他們是否能編輯該留言。  **/
    contextUserId?: string
	/** 是否要檢查新留言是否看起來像垃圾留言？  **/
    doSpamCheck?: 'true' | 'false'
	/** 該留言是否應對使用相同 urlId 的留言小工具實例的使用者顯示為「即時」。注意：會將點數成本從 1 加倍為 2。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '評論 PATCH 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** 於失敗時包含。 **/
    reason?: string
}
[inline-code-end]