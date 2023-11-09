[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

This API is used to get comments for displaying to a user. For example, it automatically filters
out unapproved or spam comments.

### Pagination

Pagination can be done in one of two ways, depending on performance requirements and use case:

1. Fastest: **Precalculated Pagination**:
   1. This is how FastComments works when you use our prebuilt widgets and clients.
   2. Clicking "next" simply increases the page count.
   3. You can think of this as being retrieved by a key-value store.
   4. In this way, simply define a `page` parameter starting at `0` and a sort direction as `direction`.
   5. Page sizes can be customized via customization rules.
2. Most Flexible: **Flexible Pagination**:
   1. In this way you can define custom `limit` and `skip` parameters. Do not pass `page`.
   2. Sort `direction` is also supported.
   3. `limit` is the total number to return after `skip` is applied.
      1. Example: set `skip = 200, limit = 100` when `page size = 100` and `page = 2`.
   4. Child comments still count in the pagination. You can get around this using the `asTree` option.

### Threads

1. When using `Precalculated Pagination`, comments are grouped by *page* and comments in threads affect the overall page.
   1. In this way, threads can be determined on the client based on `parentId`.
   2. For example, with a page with one top-level comment, and 29 replies, and setting `page=0` in the API - you will get just the top level comment and the 29 children.
   3. [Example image here illustrating multiple pages.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. When using `Flexible Pagination`, you may define a `parentId` parameter.
   1. Set this to null to only get top-level comments.
   2. Then to view threads, call the API again and pass `parentId`.
   3. A common solution is to make an API call for the top-level comments and then make parallel API calls to get comments for the children of each comment.
3. __NEW As of Feb 2023!__ Fetch as a tree using `&asTree=true`.
   1. You can think of this as `Flexible Pagination as a Tree`.
   2. Only the top-level comments count in the pagination.
   3. Set `parentId=null` to start the tree at the root (you must set `parentId`).
   4. Set `skip` and `limit` for pagination.
   5. Set `asTree` to `true`.
   6. The credits cost increases by `2x`, as our backend has to do much more work in this scenario.

### Fetching Comments in The Context of a User

The `/comments` API can be used in two contexts, for different use cases:

- For returning comments sorted and tagged with information for building your own client.
  - In this case, define a `contextUserId` query parameter.
- For fetching comments from your backend for custom integrations.
  - The platform will default to this without `contextUserId`. 

[inline-code-attrs-start title = 'Comments Precalculated Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Flexible Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Flexible Pagination in User Context'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Comments Flexible Pagination in User Context for Top Level Comments Only'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

It's possible to get the comments returned as a tree, with pagination only counting the top-level comments.

[inline-code-attrs-start title = 'Comments As-A-Tree in User Context'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

### Get Comments as a Tree, Searching by Hash Tag

It's possible to search by hashtag using the API, across your entire tenant (not limited to one page, or `urlId`).

In this example, we omit `urlId`, and we search by multiple hashtags. The API will only return comments that have all requested hashtags.

[inline-code-attrs-start title = 'Comments As-A-Tree in User Context, By Hash Tag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Comments Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
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
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
}
[inline-code-end]

### The Response

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

### Helpful Tips

#### URL ID

You probably want to use the `Comment` API with the `urlId` parameter. You can call the `Pages` API first, to see what the `urlId` values available to you look like. 

#### Anonymous Actions

For anonymous commenting you probably want to pass `anonUserId` when fetching comments, and when performing flagging and blocking.

(!) This is required for many app stores as users must be able to flag user-created content they can see, even if they are not logged in. Not doing so may cause your app to be removed from said store.

#### Comments Not Being Returned

Check that your comments are approved, and are not spam.
