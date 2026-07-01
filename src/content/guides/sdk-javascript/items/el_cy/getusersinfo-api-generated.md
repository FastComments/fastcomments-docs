Συγκεντρωτικές πληροφορίες χρηστών για έναν ενοικιαστή. Δίνεται η λίστα των userIds, επιστρέφει πληροφορίες εμφάνισης από το User / SSOUser.  
Χρησιμοποιείται από το widget σχολίων για να εμπλουτίσει τους χρήστες που μόλις εμφανίστηκαν μέσω γεγονότος παρουσίας.  
Χωρίς περισάφισμα σελίδας: η προστασία απορρήτου εφαρμόζεται ομοιόμορφα (ιδιωτικά προφίλ καλύπτονται).

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| ids | string | Ναι |  |

## Response

Επιστρέφει: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Optional fields in the response may be undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]