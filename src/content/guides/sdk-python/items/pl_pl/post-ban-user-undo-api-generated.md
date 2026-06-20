## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Przykład

[inline-code-attrs-start title = 'post_ban_user_undo Przykład'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.ban_user_undo_params import BanUserUndoParams
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    ban_user_undo_params = client.BanUserUndoParams() # BanUserUndoParams | 
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.post_ban_user_undo(ban_user_undo_params, sso=sso)
        print("The response of ModerationApi->post_ban_user_undo:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_undo: %s\n" % e)
[inline-code-end]

---