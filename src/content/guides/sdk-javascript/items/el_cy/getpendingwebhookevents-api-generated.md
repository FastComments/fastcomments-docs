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

## Απόκριση

Επιστρέφει: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_a1b2c3';
const commentId: string = 'cmt_9f8e7d';
const eventType: string = 'comment.created';
const domain: string = 'comments.acme-corp.com';
const attemptCountGT: number = 2;
const skip: number = 5;

const result: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined,
  eventType,
  undefined,
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]

---