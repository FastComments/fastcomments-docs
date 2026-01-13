## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | string | query | No |  |
| commentDeleteMode | string | query | No |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład delete_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej znajdują się przykłady dla każdej metody uwierzytelniania, użyj tego,
# które odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Skonfiguruj autoryzację klucza API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (e.g. Bearer) dla klucza API, jeśli to konieczne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = 'delete_comments_example' # str |  (opcjonalne)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.delete_tenant_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant_user: %s\n" % e)
[inline-code-end]