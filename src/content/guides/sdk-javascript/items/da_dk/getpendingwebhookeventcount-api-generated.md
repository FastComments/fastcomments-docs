## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Nej |  |
| externalId | string | Nej |  |
| eventType | string | Nej |  |
| type | string | Nej |  |
| domain | string | Nej |  |
| attemptCountGT | number | Nej |  |

## Svar

Returnerer: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3b';
const commentId: string = 'cmt_1a2b3c';
const eventType: string = 'comment.created';
const domain: string = 'news-site.com';
const attemptCountGT: number = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined, // externalId udeladt
  eventType,
  undefined, // type udeladt
  domain,
  attemptCountGT
);
[inline-code-end]

---