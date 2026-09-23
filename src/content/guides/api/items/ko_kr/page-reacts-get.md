[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

페이지의 각 반응에 대한 카운트를 반환하고, 현재 사용자가 추가한 반응을 반환합니다.

[inline-code-attrs-start title = '페이지 반응 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** SSO 객체의 URI 인코딩된 JSON입니다. 익명 사용자는 생략하세요. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: string
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 반응 ID별 카운트, 예시: {"heart": 12, "laugh": 3}. 페이지에 반응이 없을 경우 설정되지 않습니다. **/
    counts?: Record<string, number>
    /** 요청을 보낸 사용자가 추가한 반응 ID들. 사용자가 아무것도 추가하지 않은 경우 설정되지 않습니다. **/
    reactedIds?: string[]
}
[inline-code-end]