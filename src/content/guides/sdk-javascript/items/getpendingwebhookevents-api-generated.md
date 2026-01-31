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
const tenantId: string = 'tenant_7f3d2b9a';
const commentId: string = 'cmt_92a1b4';
const eventType: string = 'comment.posted';
const domain: string = 'blog.acme-corp.com';
const attemptCountGT: number = 1;
const skip: number = 0;

const response: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined,   // externalId (optional)
  eventType,
  undefined,   // type (optional)
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]
