[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags/bulk'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава додавање до 100 `HashTag` објеката одједном.

[inline-code-attrs-start title = 'Пример cURL захтева за масовно креирање HashTag-ова'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура захтева за масовно креирање HashTag-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за масовно креирање HashTag-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagBulkPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** Укључено у случају неуспеха. **/
    reason?: string
    results?: HashTagPostResponse[]; // Враћамо листу објеката HashTagPostResponse за сваки прослеђени таг.
}
[inline-code-end]

---