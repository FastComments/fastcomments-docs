## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderator_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_moderator'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderator_response import GetModeratorResponse
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z zasadami bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania, użyj tego,
# który odpowiada Twojemu przypadkowi użycia.

# Skonfiguruj autoryzację klucza API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzeba
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_moderator(tenant_id, id)
        print("The response of DefaultApi->get_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderator: %s\n" % e)
[inline-code-end]