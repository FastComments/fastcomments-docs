Επί του παρόντος online θεατές μιας σελίδας: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή αυτή τη στιγμή.
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και η προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα πλαίσιο με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή).
    after_name = 'after_name_example' # str | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απάντηση. (προαιρετικό)
    after_user_id = 'after_user_id_example' # str | Δευτερεύων δείκτης (tie-breaker): περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε ισοβαθμίες ονομάτων να μην παραλείπουν εγγραφές. (προαιρετικό)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]