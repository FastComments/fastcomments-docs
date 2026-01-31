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
(async () => {
  const tenantId: string = "tenant_9f2b1a2";
  const commentId: string = "comment_47a9d";
  const externalId: string | undefined = undefined;
  const eventType: string = "comment.published";
  const webhookType: string = "http";
  const domain: string = "comments.myapp.com";
  const attemptCountGT: number = 1;
  const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
    tenantId,
    commentId,
    externalId,
    eventType,
    webhookType,
    domain,
    attemptCountGT
  );
  console.log(result);
})();
[inline-code-end]
