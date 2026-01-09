[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

此 API 端點提供建立頁面的功能。

一個常見的使用情境是存取控制。

Notes:

- If you've commented on a comment thread, or called the API to create a `Comment`, you've already created a `Page` object! You can try fetching it via
  the `/by-url-id` `Page` route, passing in the same `urlId` passed to the comment widget.
- The `Page` structure contains some **計算得出** values.
  Currently, these are `commentCount` and `rootCommentCount`.
  They are populated automatically and cannot be set by the API. Attempting to do so will cause the API to return an error.

[inline-code-attrs-start title = '頁面 POST cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '頁面 POST 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '頁面 POST 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** 僅在失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** 僅在失敗時包含。 **/
    reason?: string
    /** 已建立的頁面。 **/
    page?: Page
}
[inline-code-end]

---