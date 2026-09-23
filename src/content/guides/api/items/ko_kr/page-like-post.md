[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

현재 사용자로서 페이지에 좋아요를 표시합니다. 각 사용자는 페이지에 한 번만 좋아요를 할 수 있습니다: 다시 좋아요를 시도하면 `already-liked` 코드와 함께 성공하며 카운트는 변경되지 않습니다.

페이지가 아직 존재하지 않으면 생성됩니다. `title`을 전달하여 페이지의 제목을 설정하거나 업데이트합니다.

[inline-code-attrs-start title = '페이지 좋아요 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** 페이지의 제목을 설정합니다. **/
    title?: string
    /** SSO 객체의 URI 인코딩된 JSON입니다. 익명 사용자는 생략합니다. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 좋아요 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 사용자가 이미 페이지에 좋아요를 표시한 경우 'already-liked'입니다. 그 외의 경우는 실패 시 포함됩니다. **/
    code?: 'already-liked' | string
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]