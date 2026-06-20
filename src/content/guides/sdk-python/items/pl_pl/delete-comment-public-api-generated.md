## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| commentId | string | path | Tak |  |
| broadcastId | string | query | Tak |  |
| editKey | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_delete_comment_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład delete_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_api_delete_comment_response import PublicAPIDeleteCommentResponse
from client.rest import ApiException
from pprint import pprint

# Ustalenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.delete_comment_public(tenant_id, comment_id, broadcast_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->delete_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_public: %s\n" % e)
[inline-code-end]