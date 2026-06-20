Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή συνδεδεμένοι. Ταξινομούνται κατά displayName.
Χρησιμοποιήστε αυτό μετά την εξάντληση του /users/online για να αποδώσετε μια ενότητα "Members".
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Page URL identifier (cleaned server-side). |
| afterName | string | query | Όχι | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | Όχι | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλεγμένη τιμή https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε πλαίσιο με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Αναγνωριστικό διεύθυνσης URL της σελίδας (καθαρίζεται από τον διακομιστή).
    after_name = 'after_name_example' # str | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. (προαιρετικό)
    after_user_id = 'after_user_id_example' # str | Διαχωριστικό ισοβαθμίας δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοβαθμίες ονομάτων να μην παραλείπουν καταχωρήσεις. (προαιρετικό)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]