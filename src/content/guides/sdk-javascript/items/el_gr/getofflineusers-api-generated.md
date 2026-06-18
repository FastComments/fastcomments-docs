Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή online. Ταξινομημένα κατά displayName.
Χρησιμοποιήστε αυτό αφού εξαντλήσετε το /users/online για να εμφανίσετε ένα τμήμα «Μέλη».
Cursor pagination on commenterName: ο διακομιστής διασχίζει το μερικό {tenantId, urlId, commenterName}
index από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| afterName | string | Όχι |  |
| afterUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---