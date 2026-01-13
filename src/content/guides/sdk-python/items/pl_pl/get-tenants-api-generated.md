## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| meta | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_tenants'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants200_response import GetTenants200Response
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania; użyj przykładu,
# który odpowiada Twojemu przypadkowi użycia.

# Skonfiguruj autoryzację klucza API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to potrzebne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (opcjonalne)
    skip = 3.4 # float |  (opcjonalne)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]