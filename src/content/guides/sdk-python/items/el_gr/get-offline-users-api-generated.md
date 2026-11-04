Προηγούμενοι σχολιαστές της σελίδας που ΔΕΝ είναι επί του παρόντος συνδεδεμένοι. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε το αυτό αφού εξαντλήσετε το /users/online για να εμφανίσετε μια ενότητα "Members".
Σελιδοποίηση με cursor στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName}
από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαραίτητο όταν ορίζεται το afterName, ώστε οι ισοβαθμίες ονομάτων να μην χάνουν εγγραφές. |

## Απάντηση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και η προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισάγετε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
    after_name = 'after_name_example' # str | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. (optional)
    after_user_id = 'after_user_id_example' # str | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαραίτητο όταν ορίζεται το afterName, ώστε οι ισοβαθμίες ονομάτων να μην χάνουν εγγραφές. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]