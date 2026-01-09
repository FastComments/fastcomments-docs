[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

此 API 端點提供建立評論的功能。

常見用例包括自訂使用者介面、整合或匯入。

注意事項：

- 如果需要，此 API 可以「即時」更新評論小工具（這會將 `creditsCost` 從 `1` 增加到 `2`）。
- 如果提供了 email，這個 API 會自動在系統中建立使用者物件。
- 嘗試儲存兩則使用不同 email 但相同使用者名稱的評論，第二則評論會發生錯誤。
- 如果您指定了 `parentId`，且子評論的 `notificationSentForParent` 為 false，**我們將會為父評論發送通知**。此操作每小時進行（我們會將通知批次一起發送以減少郵件數量）。
- 如果您想在建立使用者時發送歡迎郵件，或想發送評論驗證郵件，請在查詢參數中將 `sendEmails` 設為 `true`。
- 透過此 API 建立的評論將顯示在管理應用程式的 Analytics 與 Moderation 頁面中。
- 如果該設定已開啟，評論者名稱與評論文字中的「髒話」仍會被遮罩。
- 透過此 API 建立的評論仍可視需要進行垃圾訊息檢查。
- 若透過「自訂規則」管理頁面設定的配置（例如最大評論長度）將在此處套用。

要在評論小工具中顯示，提交的最小資料如下：

[inline-code-attrs-start title = '最小評論 POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

更實際的請求範例如下：

[inline-code-attrs-start title = '評論 POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = '評論 POST 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** 是否該評論應對在具有相同 urlId 的評論小工具實例中查看的使用者顯示為「即時」。NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '評論 POST 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** 失敗時包含。 **/
    reason?: string
    /** 建立的評論。 **/
    comment?: Comment
    /** 關聯的使用者，可能已存在或不存在。 **/
    user?: User
}
[inline-code-end]