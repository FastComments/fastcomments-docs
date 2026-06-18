## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Ne |  |
| externalId | string | Ne |  |
| eventType | string | Ne |  |
| type | string | Ne |  |
| domain | string | Ne |  |
| attemptCountGT | number | Ne |  |

## Odgovor

Vraća: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3b';
const commentId: string = 'cmt_1a2b3c';
const eventType: string = 'comment.created';
const domain: string = 'news-site.com';
const attemptCountGT: number = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined, // externalId izostavljen
  eventType,
  undefined, // type izostavljen
  domain,
  attemptCountGT
);
[inline-code-end]

---