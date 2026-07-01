Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή σε σύνδεση. Ταξινομημένα κατά displayName.  
Χρησιμοποιήστε το αυτό αφού εξαντλήσετε το /users/online για να αποδώσετε μια ενότητα "Members".  
Σελιδοποίηση κέρσορα στο commenterName: ο διακομιστής διασχίζει το μερικό {tenantId, urlId, commenterName} ευρετήριο από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Απάντηση

Επιστρέφει: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Παράδειγμα

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