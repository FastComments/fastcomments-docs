---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Nie |  |
| commentId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_manual_badges_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_manual_badges_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_manual_badges_response import GetUserManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    badges_user_id = 'badges_user_id_example' # str |  (opcjonalne)
    comment_id = 'comment_id_example' # str |  (opcjonalne)
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.get_manual_badges_for_user(badges_user_id=badges_user_id, comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_manual_badges_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges_for_user: %s\n" % e)
[inline-code-end]

---