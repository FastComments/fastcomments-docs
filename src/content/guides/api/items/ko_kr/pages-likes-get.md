[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

페이지에 대한 좋아요 수를 반환하며, 선택적으로 가장 최근 순으로 좋아요를 누른 사용자들을 반환합니다. 이는 [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get) 의 서버 측 버전입니다.

`includeUserInfo` 는 반환되는 사용자를 제어합니다:

- `public` 은 프로필 활동을 비공개로 설정한 사용자를 제외합니다.
- `include-private` 은 해당 사용자를 포함합니다. 이 옵션은 목록이 공개되지 않을 때만 사용하십시오.

익명 사용자는 절대 포함되지 않으며, 최대 100개의 좋아요가 조회됩니다. 존재하지 않는 페이지는 `likeCount` 가 `0` 이고 `users` 목록이 비어 있습니다.

[inline-code-attrs-start title = '페이지 좋아요 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### 유용한 팁

`urlId` 와 같은 값은 URI 인코딩을 해야 합니다.