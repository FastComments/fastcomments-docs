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

Retorna: [`GetPendingWebhookEventsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPendingEvents(): Promise<void> {
    const tenantId: string = "123e4567-e89b-12d3-a456-426614174000";
    const commentId: string = "cmt-987654321";
    const externalId: string = "ext-abc-123";
    const eventType: string = "comment_created";
    const type: string = "outbound";
    const domain: string = "myblog.com";
    const attemptCountGT: number = 3;
    const skip: number = 0;

    const pending: GetPendingWebhookEventsResponse1 = await getPendingWebhookEvents(
        tenantId,
        commentId,
        externalId,
        eventType,
        type,
        domain,
        attemptCountGT,
        skip
    );

    console.log(pending);
}

fetchPendingEvents();
[inline-code-end]