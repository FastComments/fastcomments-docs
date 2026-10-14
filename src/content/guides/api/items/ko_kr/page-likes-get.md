[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

페이지에 대한 좋아요 수와 현재 사용자가 해당 페이지에 좋아요를 눌렀는지 여부를 반환합니다. 아직 존재하지 않는 페이지는 `likeCount`가 `0`으로 반환됩니다.

`includeUserInfo=public`을 전달하면 페이지에 좋아요를 누른 사용자들을 최신 순으로 가져올 수 있습니다. 비공개 프로필 활동을 가진 사용자와 익명 사용자는 제외되며, 최대 100개의 좋아요가 조회됩니다. 여기서는 `public`만 허용됩니다; `include-private`를 사용하면 `422` 오류가 반환되며 API 키와 함께 [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get)에서 사용할 수 있습니다. SSO가 필요한 페이지에서는 `includeUserInfo`가 페이지를 볼 수 없는 사용자에게 `403`을 반환합니다.

[inline-code-attrs-start title = '페이지 좋아요 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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