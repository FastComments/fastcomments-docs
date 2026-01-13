## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| email | string | path | Tak |  |

## Odpowiedź

Zwraca: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_user_by_email_api_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_sso_user_by_email'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_user_by_email_api_response import GetSSOUserByEmailAPIResponse
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z zasadami bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania, użyj przykładu,
# który odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Skonfiguruj uwierzytelnianie kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    email = 'email_example' # str | 

    try:
        api_response = api_instance.get_sso_user_by_email(tenant_id, email)
        print("The response of DefaultApi->get_sso_user_by_email:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_user_by_email: %s\n" % e)
[inline-code-end]