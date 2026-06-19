Προηγούμενοι σχολιαστές της σελίδας που ΔΕΝ είναι αυτή τη στιγμή online. Ταξινομημένο κατά displayName.
Χρησιμοποιήστε το αυτό αφού εξαντλήσετε /users/online για να εμφανίσετε μια ενότητα "Μέλη".
Σελιδοποίηση με cursor στο commenterName: ο server διασχίζει το μερικό {tenantId, urlId, commenterName}
index από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| afterName | string | Όχι |  |
| afterUserId | string | Όχι |  |

## Response

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---