[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Returns the number of likes on a page, and optionally the users who liked it, most recent first. This is the server-side version of [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` controls which users are returned:

- `public` leaves out users who have made their profile activity private.
- `include-private` includes them. Only use this when the list is not shown to the public.

Anonymous users are never included, and up to 100 likes are looked up. Pages that do not exist return a `likeCount` of `0` and an empty `users` list.

[inline-code-attrs-start title = 'Page Likes cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Helpful Tip

Remember to URI Encode values like the `urlId`.
