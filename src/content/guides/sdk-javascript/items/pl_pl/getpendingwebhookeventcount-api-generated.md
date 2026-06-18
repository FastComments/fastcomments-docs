## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Nie |  |
| externalId | string | Nie |  |
| eventType | string | Nie |  |
| type | string | Nie |  |
| domain | string | Nie |  |
| attemptCountGT | number | Nie |  |

## Odpowiedź

Zwraca: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3b';
const commentId: string = 'cmt_1a2b3c';
const eventType: string = 'comment.created';
const domain: string = 'news-site.com';
const attemptCountGT: number = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined, // externalId pominięty
  eventType,
  undefined, // type pominięty
  domain,
  attemptCountGT
);
[inline-code-end]

---