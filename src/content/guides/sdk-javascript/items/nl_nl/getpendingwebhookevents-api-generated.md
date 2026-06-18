## Parameters

| Name | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Nee |  |
| externalId | string | Nee |  |
| eventType | string | Nee |  |
| type | string | Nee |  |
| domain | string | Nee |  |
| attemptCountGT | number | Nee |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPendingWebhookEvents Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const commentId: string = 'cmt_8a7d1';
const eventType: string = 'comment.created';
const domain: string = 'reviews.myshop.com';
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
[inline-code-end]

---