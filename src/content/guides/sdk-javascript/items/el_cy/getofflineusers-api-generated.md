Past commenters on the page who are NOT currently online. Sorted by displayName.  
Χρησιμοποιήστε το μετά την εξάντληση του /users/online για να εμφανίσετε μια ενότητα "Members".  
Κατάτμηση σελίδωσης με κέρσορα στο commenterName: ο διακομιστής διασχίζει το μερικό {tenantId, urlId, commenterName} δείκτη από afterName προώθηση μέσω $gt, χωρίς κόστος $skip.

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| afterName | string | Όχι |  |
| afterUserId | string | Όχι |  |

## Response

Επιστρέφει: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]