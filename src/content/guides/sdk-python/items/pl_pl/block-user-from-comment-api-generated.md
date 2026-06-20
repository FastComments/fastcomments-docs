## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |
| userId | string | query | Nie |  |
| anonUserId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`BlockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/block_success.py)

## Przykład

[inline-code-attrs-start title = 'Przykład block_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.block_from_comment_params import BlockFromCommentParams
from client.models.block_success import BlockSuccess
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Przykłady dla każdej metody uwierzytelniania znajdują się poniżej, użyj przykładu, który
# odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Skonfiguruj autoryzację klucza API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić przedrostek (np. Bearer) dla klucza API, jeśli jest potrzebny
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    block_from_comment_params = client.BlockFromCommentParams() # BlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (opcjonalne)
    anon_user_id = 'anon_user_id_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.block_user_from_comment(tenant_id, id, block_from_comment_params, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->block_user_from_comment: %s\n" % e)
[inline-code-end]