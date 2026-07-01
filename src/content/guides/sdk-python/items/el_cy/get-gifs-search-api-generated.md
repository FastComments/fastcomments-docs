## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| search | string | query | Ναι |  |
| locale | string | query | Όχι |  |
| rating | string | query | Όχι |  |
| page | number | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_gifs_search'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetGifsSearchOptions
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και έχει ως προεπιλογή https://fastcomments.com
# Δείτε το configuration.py για τη λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισαγωγή ενός context με μια παρουσία του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργία μιας παρουσίας της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str |
    search = 'search_example' # str |
    locale = 'locale_example' # str |  (προαιρετικό)
    rating = 'rating_example' # str |  (προαιρετικό)
    page = 3.4 # float |  (προαιρετικό)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, GetGifsSearchOptions(locale=locale, rating=rating, page=page))
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]