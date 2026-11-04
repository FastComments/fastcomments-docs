[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

此 API 端點提供建立頁面的功能。

常見的使用案例是存取控制。

注意事項：

- 如果您曾在留言串中留言，或呼叫 API 建立 `Comment`，那麼您已經建立了一個 `Page` 物件！您可以嘗試透過 `/by-url-id` `Page` 路由取得它，並傳入與留言小工具相同的 `urlId`。
- `Page` 結構包含一些**計算得出的**值。
  目前這些值為 `commentCount` 和 `rootCommentCount`。
  它們會自動填入，無法透過 API 設定。嘗試這樣做會導致 API 回傳錯誤。

[inline-code-attrs-start title = 'Page POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page POST 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** 失敗時包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** 失敗時包含此欄位。 **/
    reason?: string
    /** 已建立的頁面。 **/
    page?: Page
}
[inline-code-end]