## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |
| userId | string | query | Nie |  |
| anonUserId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/unblock_success.py)

## Przykład

[inline-code-attrs-start title = 'Przykład un_block_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.un_block_from_comment_params import UnBlockFromCommentParams
from client.models.unblock_success import UnblockSuccess
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania — użyj przykładu,
# który odpowiada Twojemu przypadkowi użycia.

# Konfiguracja autoryzacji klucza API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzeba
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    un_block_from_comment_params = client.UnBlockFromCommentParams() # UnBlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (opcjonalne)
    anon_user_id = 'anon_user_id_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.un_block_user_from_comment(tenant_id, id, un_block_from_comment_params, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->un_block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->un_block_user_from_comment: %s\n" % e)
[inline-code-end]