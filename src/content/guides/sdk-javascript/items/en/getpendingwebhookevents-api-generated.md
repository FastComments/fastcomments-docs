## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | No |  |
| externalId | string | No |  |
| eventType | string | No |  |
| type | string | No |  |
| domain | string | No |  |
| attemptCountGT | number | No |  |
| skip | number | No |  |

## Response

Returns: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEvents Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_12345';
const commentId: string | undefined = 'cmt-98765';
const eventType: string | undefined = 'comment.created';
const domain: string | undefined = 'reviews.example.com';
const attemptCountGT: number | undefined = 2;
const skip: number | undefined = 0;
const response: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(tenantId, commentId, undefined, eventType, undefined, domain, attemptCountGT, skip);
[inline-code-end]
