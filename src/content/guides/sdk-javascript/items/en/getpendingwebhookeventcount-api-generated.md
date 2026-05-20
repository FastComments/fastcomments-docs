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

## Response

Returns: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEventCount Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentId: string | undefined = 'cmt-98765';
const externalId: string | undefined = 'order-5643';
const eventType: string | undefined = 'comment.created';
const typeParam: string | undefined = 'webhook';
const domain: string | undefined = 'app.mycompany.com';
const attemptCountGT: number | undefined = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  externalId,
  eventType,
  typeParam,
  domain,
  attemptCountGT
);
[inline-code-end]
