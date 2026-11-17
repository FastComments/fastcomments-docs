## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Όχι |  |
| externalId | string | Όχι |  |
| eventType | string | Όχι |  |
| type | string | Όχι |  |
| domain | string | Όχι |  |
| attemptCountGT | number | Όχι |  |
| skip | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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