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
const tenantId: string = "tenant_78b2f1";
const commentId: string = "cmt_0042";
const eventType: string = "comment.created";
const domain: string = "blog.example.com";
const attemptCountGT: number = 1;
const skip: number = 0;

const pending: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined, // externalId
  eventType,
  undefined, // type
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]