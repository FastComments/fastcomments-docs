## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| search | string | query | Tak |  |
| locale | string | query | Nie |  |
| rating | string | query | Nie |  |
| page | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_gifs_search'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz plik configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    search = 'search_example' # str | 
    locale = 'locale_example' # str |  (opcjonalne)
    rating = 'rating_example' # str |  (opcjonalne)
    page = 3.4 # float |  (opcjonalne)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]