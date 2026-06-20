Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτήν τη στιγμή online. Ταξινομούνται κατά displayName.
Χρησιμοποιήστε αυτό αφού εξαντλήσετε το /users/online για να εμφανίσετε μια ενότητα "Μέλη".
Cursor pagination στο commenterName: ο διακομιστής διασχίζει τον μερικό δείκτη {tenantId, urlId, commenterName}
index από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | No | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Επιλύτης δεσμών cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName είναι ορισμένο ώστε οι ισοβαθμίες στο όνομα να μην παραλείπονται καταχωρήσεις. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και από προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Μπείτε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
    after_name = 'after_name_example' # str | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. (προαιρετικό)
    after_user_id = 'after_user_id_example' # str | Επιλύτης δεσμών cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName είναι ορισμένο ώστε οι ισοβαθμίες στο όνομα να μην παραλείπονται καταχωρήσεις. (προαιρετικό)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]