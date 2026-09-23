[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

현재 사용자의 페이지에 대한 리액션 중 하나를 제거합니다. 사용자가 해당 리액션을 추가하지 않은 경우, 요청은 `no-react` 코드와 함께 성공하며 카운트는 변경되지 않습니다.

[inline-code-attrs-start title = '페이지 리액션 삭제 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '페이지 리액션 삭제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** 리액션 ID. **/
    id: string
    /** URI 인코딩된 JSON 형태의 SSO 객체. 익명 사용자는 생략합니다. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 리액션 삭제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react'는 사용자가 이 리액션을 추가하지 않았을 때 반환됩니다. 그 외의 경우는 실패 시 포함됩니다. **/
    code?: 'no-react' | string
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]