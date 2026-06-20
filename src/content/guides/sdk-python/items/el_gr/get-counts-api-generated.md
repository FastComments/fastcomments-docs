## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_count_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_counts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_count_response import GetBannedUsersCountResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένο το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str | (προαιρετικό)

    try:
        api_response = api_instance.get_counts(sso=sso)
        print("The response of ModerationApi->get_counts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_counts: %s\n" % e)
[inline-code-end]