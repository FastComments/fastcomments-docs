[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

페이지의 각 반응에 대한 카운트를 반환하고, 현재 사용자가 추가한 반응을 반환합니다.

`includeUserInfo=public`을 전달하면 반응한 사용자도 가져올 수 있으며, 반응당 하나의 항목으로 반응 ID와 함께 가장 최신 순으로 반환됩니다. 개인 프로필 활동을 가진 사용자와 익명 사용자는 제외되며, 최대 100개의 반응이 조회됩니다. 여기서는 `public`만 허용됩니다; `include-private`는 `422`를 반환하며 API 키와 함께 [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get)에서 사용할 수 있습니다. SSO가 필요한 페이지에서는 `includeUserInfo`가 페이지를 볼 수 없는 사용자에게 `403`을 반환합니다.

[inline-code-attrs-start title = '페이지 반응 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who reacted. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** The reaction ids the user making the request has added. Not set when they have added none. **/
    reactedIds?: string[]
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]