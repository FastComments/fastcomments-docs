## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Non |  |
| externalId | string | Non |  |
| eventType | string | Non |  |
| type | string | Non |  |
| domain | string | Non |  |
| attemptCountGT | number | Non |  |
| skip | number | Non |  |

## Réponse

Renvoie : [`GetPendingWebhookEventsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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