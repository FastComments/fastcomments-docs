## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_search_suggest'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetSearchSuggestOptions
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και προεπιλεγμένος στην https://fastcomments.com
# Δείτε το configuration.py για τη λίστα όλων των υποστηριζόμενων παραμέτρων παραμετροποίησης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισαγωγή ενός πλαισίου με ένα στιγμιότυπο του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργία ενός στιγμιότυπου της κλάσης API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_search_suggest(tenant_id, GetSearchSuggestOptions(text_search=text_search, sso=sso))
        print("The response of ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]