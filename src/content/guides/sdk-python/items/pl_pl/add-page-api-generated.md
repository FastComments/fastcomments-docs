## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Response

Zwraca: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_page_api_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład add_page'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_page_api_response import AddPageAPIResponse
from client.models.create_api_page_data import CreateAPIPageData
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej znajdują się przykłady dla każdej metody uwierzytelniania, użyj przykładu,
# który odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Skonfiguruj autoryzację klucza API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_page_data = client.CreateAPIPageData() # CreateAPIPageData | 

    try:
        api_response = api_instance.add_page(tenant_id, create_api_page_data)
        print("The response of DefaultApi->add_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_page: %s\n" % e)
[inline-code-end]