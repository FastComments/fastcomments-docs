[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

返回页面的点赞数量，以及当前用户是否已点赞。不存在的页面返回 `likeCount` 为 `0`。

传入 `includeUserInfo=public` 可同时获取点赞该页面的用户，按最近的先后顺序。私人资料活动的用户和匿名用户将被排除，最多查询 100 条点赞。此处仅接受 `public`；`include-private` 会返回 `422`，并可通过带 API 密钥的 [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) 使用。在需要 SSO 的页面上，`includeUserInfo` 对无法查看页面的用户返回 `403`。

[inline-code-attrs-start title = '页面点赞 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '页面点赞请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = '页面点赞响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
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