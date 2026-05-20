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
  const tenantId: string = "tenant_8a7b42";
  const commentId: string | undefined = "cmt-9021";
  const externalId: string | undefined = "order-4452";
  const eventType: string | undefined = "payment.succeeded";
  const typeParam: string | undefined = "comment";
  const domain: string | undefined = "shop.merchant.com";
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
  console.log(result);
})();
[inline-code-end]
