[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

현재 사용자의 페이지 좋아요를 제거합니다. 사용자가 페이지에 좋아요를 누르지 않은 경우, 요청은 `not-liked` 코드와 함께 성공하며 카운트는 변경되지 않습니다.

[inline-code-attrs-start title = '페이지 좋아요 취소 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 취소 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI 인코딩된 JSON 형태의 SSO 객체. 익명 사용자는 생략하세요. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 취소 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked'는 사용자가 페이지에 좋아요를 누르지 않았을 때. 실패 시에만 포함됩니다. **/
    code?: 'not-liked' | string
    /** 실패 시에 포함됩니다. **/
    reason?: string
}
[inline-code-end]