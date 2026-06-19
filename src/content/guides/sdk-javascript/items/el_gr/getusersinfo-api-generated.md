---
Μαζικές πληροφορίες χρηστών για έναν tenant. Με δεδομένα τα userIds, επιστρέφει πληροφορίες εμφάνισης από User / SSOUser.
Χρησιμοποιείται από το widget σχολίων για να εμπλουτίσει χρήστες που μόλις εμφανίστηκαν μέσω ενός γεγονότος παρουσίας.
Χωρίς πλαίσιο σελίδας: η ιδιωτικότητα εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ αποκρύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Απόκριση

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// Η getUsersInfo απαιτεί μόνο tenantId και ids; οι προαιρετικές παράμετροι δεν έχουν εφαρμογή εδώ.
[inline-code-end]

---