---
[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags/bulk'; creditsCost = 1; api-resource-header-end]

此路由允許一次新增最多 100 個 `HashTag` 物件。

[inline-code-attrs-start title = 'HashTag 批次建立 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/hash-tags/bulk?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "tags": [
        {
            "tag": "#example",
            "url": "https://example.com/some-page"
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag 批次建立請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag 批次建立回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagBulkPostResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** 失敗時會包含。 **/
    reason?: string
    results?: HashTagPostResponse[]; // 我們會為每個提供的標籤回傳一個 HashTagPostResponse 物件的清單。
}
[inline-code-end]

---