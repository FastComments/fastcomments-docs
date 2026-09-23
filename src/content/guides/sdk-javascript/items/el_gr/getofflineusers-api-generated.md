---
Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή online. Ταξινομημένα κατά displayName.  
Χρησιμοποιήστε το αυτό μετά την εξάντληση του /users/online για να αποδώσετε μια ενότητα "Members".  
Σελιδοποίηση με κέρσορα στο commenterName: ο διακομιστής διασχίζει το μερικό {tenantId, urlId, commenterName} ευρετήριο από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| afterName | string | Όχι |  |
| afterUserId | string | Όχι |  |

## Response

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]

---