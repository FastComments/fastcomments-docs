## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_ban_status_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comment_ban_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_ban_status_response import GetCommentBanStatusResponse
from client.rest import ApiException
from pprint import pprint

# Definiowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.get_comment_ban_status(comment_id, sso=sso)
        print("The response of ModerationApi->get_comment_ban_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_comment_ban_status: %s\n" % e)
[inline-code-end]