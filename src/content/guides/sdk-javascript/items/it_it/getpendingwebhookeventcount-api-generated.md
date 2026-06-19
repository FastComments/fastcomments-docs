## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | No |  |
| externalId | string | No |  |
| eventType | string | No |  |
| type | string | No |  |
| domain | string | No |  |
| attemptCountGT | number | No |  |

## Risposta

Restituisce: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCountResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'd5c7e8b4-3a1f-4b2e-9f6c-12ab34cd56ef';
  const commentId: string = 'cmt_000842';
  const externalId: string = 'post-77f4';
  const eventType: string = 'comment.updated';
  const typeParam: string = 'delivery';
  const domain: string = 'myblog.example.net';
  const attemptCountGT: number = 2;

  const result: GetPendingWebhookEventCountResponse = await getPendingWebhookEventCount(
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

---