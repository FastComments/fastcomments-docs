[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

페이지의 각 반응에 대한 카운트를 반환하고, 선택적으로 반응한 사용자를 반환합니다. 각 반응은 반응 ID와 함께 최신 순으로 하나의 항목으로 제공됩니다. 이는 [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get)의 서버 측 버전입니다.

`includeUserInfo`는 반환되는 사용자를 제어합니다:

- `public`은 프로필 활동을 비공개로 설정한 사용자를 제외합니다.
- `include-private`는 해당 사용자를 포함합니다. 이 옵션은 목록이 공개되지 않을 때만 사용하십시오.

익명 사용자는 절대 포함되지 않으며, 최대 100개의 반응을 조회합니다. 존재하지 않는 페이지는 `counts`가 없고 `users` 목록이 비어 있습니다.

[inline-code-attrs-start title = '페이지 반응 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** 제외하면 카운트만 가져옵니다. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 반응 ID당 카운트, 예: {"heart": 12, "laugh": 3}. 페이지에 반응이 없을 경우 설정되지 않습니다. **/
    counts?: Record<string, number>
    /** 각 반응에 대한 항목 하나씩, 최신 순. includeUserInfo가 전달된 경우에만 설정됩니다. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** 반응 ID, 예: "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### 유용한 팁

`urlId`와 같은 값은 URI 인코딩해야 합니다.