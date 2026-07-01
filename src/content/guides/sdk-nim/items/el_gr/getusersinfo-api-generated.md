---  
Συγκεντρωτικές πληροφορίες χρήστη για έναν ενοικιαστή. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από το User / SSOUser.  
Χρησιμοποιείται από το widget σχολίων για να εμπλουτίσει τους χρήστες που μόλις εμφανίστηκαν μέσω ενός γεγονότος παρουσίας.  
Χωρίς περιβάλλον σελίδας: η ιδιωτικότητα εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ είναι μηδενισμένα).  

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| ids | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")  
if usersInfoOpt.isSome:  
  let usersInfo = usersInfoOpt.get()  
[inline-code-end]  

---