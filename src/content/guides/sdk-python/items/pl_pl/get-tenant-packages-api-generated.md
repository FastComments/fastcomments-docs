## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Odpowiedź

Zwraca: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_packages200_response.py)

## Przykład

[inline-code-attrs-start title = 'get_tenant_packages Przykład'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_packages200_response import GetTenantPackages200Response
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej zamieszczono przykłady dla każdej metody uwierzytelniania, użyj tego,
# który odpowiada Twojemu przypadkowi użycia.

# Konfiguruj uwierzytelnianie kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to potrzebne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (opcjonalne)

    try:
        api_response = api_instance.get_tenant_packages(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_packages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_packages: %s\n" % e)
[inline-code-end]