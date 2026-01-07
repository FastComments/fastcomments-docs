[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

API זה משתמש בעימוד, המסופק על ידי פרמטר השאילתה `page`. האשטאגים מוחזרים בעמודים של `100`, ממוינים לפי `tag`.

[inline-code-attrs-start title = 'דוגמת cURL להאשטאג'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת האשטאג'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The page to fetch, starting with 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת האשטאג'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The hashtags! **/
    hashTags: HashTag[]
}
[inline-code-end]
