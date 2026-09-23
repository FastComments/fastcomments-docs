## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Não |  |
| externalId | string | Não |  |
| eventType | string | Não |  |
| type | string | Não |  |
| domain | string | Não |  |
| attemptCountGT | number | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "123e4567-e89b-12d3-a456-426614174000";
const commentId: string = "cmt_987654321";
const externalId: string = "ext_abc123";
const eventType: string = "comment_created";
const type: string = "outbound";
const domain: string = "myblog.com";
const attemptCountGT: number = 2;
const skip: number = 10;

const pendingEvents: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(
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