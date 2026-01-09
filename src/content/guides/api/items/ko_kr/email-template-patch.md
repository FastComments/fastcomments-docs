[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 id와 업데이트할 속성만 지정하여 이메일 템플릿을 업데이트할 수 있는 기능을 제공합니다.

템플릿을 생성할 때와 동일한 모든 유효성 검사가 적용된다는 점에 유의하세요. 예를 들어:

- 템플릿은 렌더링되어야 합니다. 각 업데이트 시 이것이 확인됩니다.
- 동일한 도메인에 대해 중복된 템플릿을 가질 수 없습니다(그렇지 않으면 하나가 조용히 무시됩니다).

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 업데이트된 이메일 템플릿. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

---