## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Nee |  |
| externalId | string | Nee |  |
| eventType | string | Nee |  |
| type | string | Nee |  |
| domain | string | Nee |  |
| attemptCountGT | number | Nee |  |
| skip | number | Nee |  |

## Response

Retourneert: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPendingWebhookEvents Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_a1b2c3';
const commentId: string = 'cmt_9f8e7d';
const eventType: string = 'comment.created';
const domain: string = 'comments.acme-corp.com';
const attemptCountGT: number = 2;
const skip: number = 5;

const result: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(
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