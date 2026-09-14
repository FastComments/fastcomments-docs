[api-resource-header-start name = 'FeedPost'; route = 'PATCH /api/v1/feed-posts/:id'; creditsCost = 1; api-resource-header-end]

Този маршрут актуализира един `FeedPost`. Изпратете само полетата, които искате да промените.

[inline-code-attrs-start title = 'Пример за cURL за актуализиране на FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/feed-posts/some-post-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "title": "Release 2.0.1 is out",
    "tags": ["releases", "hotfix"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за актуализиране на FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchQueryParams {
    tenantId: string
    API_KEY: string
}

/** Any writable field from the FeedPost structure. **/
type FeedPostPatchBody = Partial<Pick<FeedPost, 'title' | 'contentHTML' | 'fromUserDisplayName' | 'tags' | 'weight' | 'meta' | 'media' | 'links'>>
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за актуализиране на FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'internal'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]