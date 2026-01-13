---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | zapytanie | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład create_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user200_response import CreateTenantUser200Response
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby poznać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania — użyj tego,
# który spełnia Twoje potrzeby uwierzytelniania.

# Konfiguracja uwierzytelniania kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniższą linię, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzeba
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_user_body = client.CreateTenantUserBody() # CreateTenantUserBody | 

    try:
        api_response = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
        print("The response of DefaultApi->create_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_user: %s\n" % e)
[inline-code-end]

---