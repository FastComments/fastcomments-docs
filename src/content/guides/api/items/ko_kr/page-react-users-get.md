[api-resource-header-start name = '페이지 리액트 사용자'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

페이지에 반응을 추가한 사용자의 이름을 알파벳 순으로 반환합니다. 최대 100개의 반응을 조회하며, 익명 사용자는 포함되지 않습니다.

[inline-code-attrs-start title = '페이지 리액트 사용자 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '페이지 리액트 사용자 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** 리액션 ID. **/
    id: string
    /** 귀하의 SSO 객체의 URI 인코딩된 JSON. 익명 사용자는 생략하십시오. **/
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