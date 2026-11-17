## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Nie |  |
| externalId | string | Nie |  |
| eventType | string | Nie |  |
| type | string | Nie |  |
| domain | string | Nie |  |
| attemptCountGT | number | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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