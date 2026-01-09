[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates/render'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 이메일 템플릿을 미리 볼 수 있는 기능을 제공합니다.

[inline-code-attrs-start title = '최소 EmailTemplate 미리보기 POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates/render?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'does-not-render';
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 성공 시 렌더링된 HTML. **/
    html?: string
}
[inline-code-end]

---