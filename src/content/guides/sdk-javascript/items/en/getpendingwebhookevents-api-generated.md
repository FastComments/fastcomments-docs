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
(async () => {
  const tenantId: string = 'd6e8f3a2-4b9f-4c3b-8123-1f2e3d4c5b6a';
  const commentId: string = 'cmt_987654321';
  const eventType: string = 'COMMENT_CREATED';
  const domain: string = 'comments.prod.example.com';
  const attemptCountGT: number = 1;
  const skip: number = 0;

  const result: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
    tenantId,
    commentId,
    undefined,
    eventType,
    undefined,
    domain,
    attemptCountGT,
    skip
  );

  console.log(result);
})();
[inline-code-end]
