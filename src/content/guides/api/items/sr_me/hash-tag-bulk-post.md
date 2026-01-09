[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags/bulk'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава додавање до 100 `HashTag` објеката одједном.

[inline-code-attrs-start title = 'HashTag Масовно креирање cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура захтјева за масовно креирање HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за масовно креирање HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagBulkPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    results?: HashTagPostResponse[]; // Враћамо листу HashTagPostResponse објеката за сваки достављени таг.
}
[inline-code-end]

---