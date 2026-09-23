구독 취소 (REST 훅 구독 취소). 이 API를 통해 생성된 구독만 여기에서 삭제할 수 있습니다; 대시보드에서 관리되는 웹훅은 대시보드에서 편집됩니다.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteWebhook 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const webhookId: string = "wh_98765";

const result: APIEmptyResponse = await deleteWebhook(tenantId, webhookId);
[inline-code-end]

---