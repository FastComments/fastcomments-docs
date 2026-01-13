## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | zapytanie | Tak |  |
| id | string | ścieżka | Tak |  |

## Odpowiedź

Zwraca: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_user_by_id_api_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_sso_user_by_id'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_user_by_id_api_response import GetSSOUserByIdAPIResponse
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z zasadami bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania — użyj
# tego, który odpowiada Twojemu przypadkowi użycia.

# Skonfiguruj autoryzację kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_sso_user_by_id(tenant_id, id)
        print("The response of DefaultApi->get_sso_user_by_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_user_by_id: %s\n" % e)
[inline-code-end]