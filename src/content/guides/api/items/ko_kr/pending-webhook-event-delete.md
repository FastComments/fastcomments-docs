[api-resource-header-start name = 'PendingWebhookEvent'; route = 'DELETE /api/v1/pending-webhook-events/:id'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 `PendingWebhookEvent`를 삭제할 수 있습니다.

대량 삭제가 필요한 경우, 페이징을 사용하여 GET API를 호출한 다음 이 API를 순차적으로 호출하세요.

[inline-code-attrs-start title = 'PendingWebhookEvent 삭제 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent 삭제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent 삭제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]