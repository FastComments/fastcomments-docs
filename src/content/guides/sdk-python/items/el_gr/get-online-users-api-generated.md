Αυτήν τη στιγμή online θεατές μιας σελίδας: άτομα των οποίων η συνεδρία websocket είναι εγγεγραμμένη στη σελίδα αυτή αυτή τη στιγμή. Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL της σελίδας (καθαρίζεται στην πλευρά του διακομιστή). |
| afterName | string | query | No | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | No | Διαιτητής δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε ισοβαθμίες ονομάτων να μην αφαιρούν καταχωρήσεις. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και η προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για μια λίστα όλων των υποστηριζόμενων παραμέτρων ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Αναγνωριστικό URL της σελίδας (καθαρίζεται στην πλευρά του διακομιστή).
    after_name = 'after_name_example' # str | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. (προαιρετικό)
    after_user_id = 'after_user_id_example' # str | Διαιτητής δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε ισοβαθμίες ονομάτων να μην αφαιρούν καταχωρήσεις. (προαιρετικό)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]