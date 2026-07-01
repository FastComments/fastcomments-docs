## Parameters

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Response

Επιστρέφει: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_trending_response.py)

## Example

[inline-code-attrs-start title = 'get_gifs_trending Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetGifsTrendingOptions
from client.models.get_gifs_trending_response import GetGifsTrendingResponse
from client.rest import ApiException
from pprint import pprint

# Ο ορισμός του host είναι προαιρετικός και προεπιλεγμένος στο https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισαγάγετε ένα context με μια παρουσία του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε μια παρουσία της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    locale = 'locale_example' # str |  (προαιρετικό)
    rating = 'rating_example' # str |  (προαιρετικό)
    page = 3.4 # float |  (προαιρετικό)

    try:
        api_response = api_instance.get_gifs_trending(tenant_id, GetGifsTrendingOptions(locale=locale, rating=rating, page=page))
        print("The response of PublicApi->get_gifs_trending:\n")
        pprint(api_response)
    except Exception as e:
        # Εξαίρεση κατά την κλήση του PublicApi->get_gifs_trending: %s\n
        print("Exception when calling PublicApi->get_gifs_trending: %s\n" % e)
[inline-code-end]