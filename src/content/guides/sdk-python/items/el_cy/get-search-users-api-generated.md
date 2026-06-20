## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| value | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_user_search_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_search_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_user_search_response import ModerationUserSearchResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένος στο https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.get_search_users(value=value, sso=sso)
        print("The response of ModerationApi->get_search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_users: %s\n" % e)
[inline-code-end]