Πληροφορίες πολλαπλών χρηστών για έναν ενοικιαστή. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από User / SSOUser. Χρησιμοποιείται από το widget σχολίων για να εμπλουτίσει χρήστες που εμφανίστηκαν μέσω ενός γεγονότος παρουσίας. Χωρίς συμφραζόμενο σελίδας: η ιδιωτικότητα επιβάλλεται ομοιόμορφα (τα ιδιωτικά προφίλ αποκρύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Απάντηση

Επιστρέφει: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Προαιρετικά πεδία στην απάντηση μπορεί να είναι undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]