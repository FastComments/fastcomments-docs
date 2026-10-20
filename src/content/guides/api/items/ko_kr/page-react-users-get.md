---
[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

페이지에 반응을 추가한 사용자의 이름을 알파벳 순으로 반환합니다. 최대 100개의 반응을 조회하며, 익명 사용자와 비공개 프로필 활동을 가진 사용자는 포함되지 않습니다. SSO가 필요한 페이지에서는 페이지를 볼 수 없는 사용자가 `403`과 코드 `sso-required` 또는 `page-forbidden`을 받습니다.

아바타와 사용자 ID도 가져오려면 `includeUserInfo=public`와 함께 [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) 를 사용하세요.

[inline-code-attrs-start title = '페이지 리액트 사용자 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '페이지 리액트 사용자 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** 반응 ID. **/
    id: string
    /** SSO 객체의 URI 인코딩된 JSON. 익명 사용자는 생략하세요. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 리액트 사용자 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: string
    /** 실패 시 포함됩니다. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---