## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Hayır |  |
| externalId | string | Hayır |  |
| eventType | string | Hayır |  |
| type | string | Hayır |  |
| domain | string | Hayır |  |
| attemptCountGT | number | Hayır |  |
| skip | number | Hayır |  |

## Yanıt

Döndürür: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getPendingWebhookEvents Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9b3f7c';
const commentId: string | undefined = undefined;
const externalId: string | undefined = 'external-572a';
const eventType: string | undefined = 'comment.updated';
const type: string | undefined = 'outbound';
const domain: string | undefined = 'reviews.example.com';
const attemptCountGT: number | undefined = 1;
const skip: number | undefined = 20;

const result: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  externalId,
  eventType,
  type,
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]

---