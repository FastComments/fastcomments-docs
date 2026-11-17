## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
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

Retorna: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a2f4d3c8b9e1a2b3c4d5e6f';

const pendingEvents: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(tenantId, {
    commentId: 'abc123def456',
    eventType: 'comment-created',
    attemptCountGT: 2,
    skip: 0
});

console.log(`Found ${pendingEvents.items.length} pending webhook events`);
[inline-code-end]