[api-resource-header-start name = 'FeedPost'; route = 'PATCH /api/v1/feed-posts/:id'; creditsCost = 1; api-resource-header-end]

נתיב זה מעדכן פוסט יחיד `FeedPost`. שלח רק את השדות שברצונך לשנות.

[inline-code-attrs-start title = 'דוגמת cURL לעדכון FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/feed-posts/some-post-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "title": "Release 2.0.1 is out",
    "tags": ["releases", "hotfix"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת עדכון FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchQueryParams {
    tenantId: string
    API_KEY: string
}

/** כל שדה שניתן לכתיבה ממבנה FeedPost. **/
type FeedPostPatchBody = Partial<Pick<FeedPost, 'title' | 'contentHTML' | 'fromUserDisplayName' | 'tags' | 'weight' | 'meta' | 'media' | 'links'>>
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת עדכון FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchResponse {
    status: 'success' | 'failed'
    /** כלול במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'internal'
    /** כלול במקרה של כשל. **/
    reason?: string
}
[inline-code-end]