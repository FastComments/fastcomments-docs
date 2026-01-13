---
## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Não |  |
| externalId | string | Não |  |
| eventType | string | Não |  |
| type | string | Não |  |
| domain | string | Não |  |
| attemptCountGT | number | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_78b2f1";
const commentId: string = "cmt_0042";
const eventType: string = "comment.created";
const domain: string = "blog.example.com";
const attemptCountGT: number = 1;
const skip: number = 0;

const pending: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined, // externalId (não definido)
  eventType,
  undefined, // type (não definido)
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]

---