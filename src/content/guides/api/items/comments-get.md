[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]


### Pagination

Pagination can be done in one of two ways, depending on performance requirements and use case:

1. Fastest: **Precalculated Pagination**:
   1. This is how FastComments works when you use our prebuilt widgets and clients.
   2. Clicking "next" simply increases the page count.
   3. You can think of this as being retrieved by a key-value store.
   4. In this way, simply define a `page` parameter starting at `0` and a sort direction as `direction`.
2. Most Flexible: **Flexible Pagination**:
   1. In this way you can define custom `limit` and `skip` parameters.
   2. Sort `direction` is also supported.
   3. `limit` is the total number to return after `skip` is applied.
      1. Example: set `skip = 200, limit = 100` when `page size = 100` and `page = 2`.

### Threads

1. When using `Precalculated Pagination`, comments are grouped by *page* and comments in threads affect the overall page.
   1. In this way, threads can be determined on the client based on `parentId`.
   2. For example, with a page with one top-level comment, and 29 replies, and setting `page=0` in the API - you will get just the top level comment and the 29 children.
   3. [Example image here illustrating multiple pages.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. When using `Flexible Pagination`, you may define a `parentId` parameter.
   1. Set this to null to only get top-level comments.
   2. Then to view threads, call the API again and pass `parentId`.
   3. A common solution is to make an API call for the top-level comments and then make parallel API calls to get comments for the children of each comment.

### Fetching Comments in The Context of a User

The `/comments` API can be used in two contexts, for different use cases:

2. For returning comments sorted and tagged with information for building your own client.
   1. In this case, define a `contextUserId` query parameter.
3. For fetching comments from your backend for custom integrations.
   1. The platform will default to this without `contextUserId`. 

[inline-code-attrs-start title = 'Comments Precalculated Pagination cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Flexible Pagination cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20=limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Flexible Pagination in User Context cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20=limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Flexible Pagination in User Context for Top Level Comments Only cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20=limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments. **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For fetching child comments. **/
    parentId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Comments Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

#### Helpful Tip

The `Comment` API requires a `urlId`. You can call the `Pages` API first, to see what the `urlId` values available to you
look like.
