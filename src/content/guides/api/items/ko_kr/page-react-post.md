[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

현재 사용자로서 페이지에 반응을 추가합니다. 사용자는 각 반응 ID를 한 번만 추가할 수 있습니다: 다시 추가하면 `already-reacted` 코드와 함께 성공하며 카운트가 변경되지 않습니다. 사용자는 같은 페이지에 여러 다른 반응을 추가할 수 있습니다.

반응 ID는 사용자가 선택하며 최대 36자까지 가능합니다. 페이지가 아직 존재하지 않으면 생성됩니다. 페이지의 제목을 설정하거나 업데이트하려면 `title`을 전달하세요.

[inline-code-attrs-start title = '페이지 반응 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** 반응 ID, 최대 36자. **/
    id: string
    /** 페이지의 제목을 설정합니다. **/
    title?: string
    /** 귀하의 SSO 객체를 URI 인코딩한 JSON입니다. 익명 사용자는 생략하세요. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 반응 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted'는 사용자가 이미 이 반응을 추가했을 때입니다. 'react-id-too-long' (HTTP 422)는 ID가 36자를 초과했을 때입니다. 그 외의 경우는 실패 시 포함됩니다. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]