## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład create_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant200_response import CreateTenant200Response
from client.models.create_tenant_body import CreateTenantBody
from client.rest import ApiException
from pprint import pprint

# Ustalenie hosta jest opcjonalne i domyślnie to https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania; użyj przykładu, który
# odpowiada Twojemu przypadkowi użycia.

# Konfiguracja autoryzacji kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_body = client.CreateTenantBody() # CreateTenantBody | 

    try:
        api_response = api_instance.create_tenant(tenant_id, create_tenant_body)
        print("The response of DefaultApi->create_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant: %s\n" % e)
[inline-code-end]