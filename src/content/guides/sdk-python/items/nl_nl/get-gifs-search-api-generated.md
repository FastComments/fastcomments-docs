## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| search | string | query | Ja |  |
| locale | string | query | Nee |  |
| rating | string | query | Nee |  |
| page | number | query | Nee |  |

## Response

Retourneert: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## Example

[inline-code-attrs-start title = 'get_gifs_search voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetGifsSearchOptions
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst met alle ondersteunde configuratie‑parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Voer een context in met een instantie van de API‑client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    search = 'search_example' # str | 
    locale = 'locale_example' # str |  (optional)
    rating = 'rating_example' # str |  (optional)
    page = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, GetGifsSearchOptions(locale=locale, rating=rating, page=page))
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]