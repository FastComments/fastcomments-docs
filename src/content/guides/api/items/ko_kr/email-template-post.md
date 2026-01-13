[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 이메일 템플릿을 생성하는 기능을 제공합니다.

Notes:

- 같은 도메인에서는 동일한 `emailTemplateId`를 가진 템플릿을 여러 개 가질 수 없습니다.
- 하지만 와일드카드 템플릿(`domain` = `*`)과 동일한 `emailTemplateId`에 대한 도메인별 템플릿을 동시에 가질 수 있습니다.
- 여러 도메인이 있거나 테스트용으로 특정 템플릿을 사용하려는 경우(예: `domain`을 `localhost`로 설정) 에만 `domain`을 지정하면 됩니다.
- 만약 `domain`을 지정하면 해당 값은 `DomainConfig`와 일치해야 합니다. 오류가 발생하면 유효한 도메인 목록이 제공됩니다.
- 템플릿 문법은 EJS이며 500ms 타임아웃으로 렌더링됩니다. 렌더링의 P99는 <5ms 이므로 500ms에 도달하면 문제가 있는 것입니다.
- **저장하려면 템플릿이 주어진 `testData`로 렌더링되어야 합니다.** 렌더링 오류는 집계되어 대시보드에 보고됩니다(곧 API를 통해서도 제공될 예정입니다). 

템플릿을 추가하는 데 필요한 최소 데이터는 다음과 같습니다:

[inline-code-attrs-start title = '최소 EmailTemplate POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

사이트별 템플릿을 원할 경우 `domain`을 정의할 수 있습니다:

[inline-code-attrs-start title = 'EmailTemplate POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
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
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 생성된 템플릿. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]