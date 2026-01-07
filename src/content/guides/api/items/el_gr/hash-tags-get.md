[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Αυτό το API χρησιμοποιεί σελιδοποίηση, που παρέχεται από την παράμετρο ερωτήματος `page`. Τα HashTags επιστρέφονται σε σελίδες των `100`, ταξινομημένα κατά `tag`.

[inline-code-attrs-start title = 'HashTag cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The page to fetch, starting with 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
