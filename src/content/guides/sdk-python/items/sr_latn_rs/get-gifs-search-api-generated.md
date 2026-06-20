## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| search | string | query | Da |  |
| locale | string | query | Ne |  |
| rating | string | query | Ne |  |
| page | number | query | Ne |  |

## Odgovor

Vraća: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## Primer

[inline-code-attrs-start title = 'get_gifs_search Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    search = 'search_example' # str | 
    locale = 'locale_example' # str |  (neobavezno)
    rating = 'rating_example' # str |  (neobavezno)
    page = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]