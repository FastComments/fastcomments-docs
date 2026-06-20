---
## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_moderate_get_user_ban_preferences_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_user_ban_preference'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_moderate_get_user_ban_preferences_response import APIModerateGetUserBanPreferencesResponse
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
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.get_user_ban_preference(sso=sso)
        print("The response of ModerationApi->get_user_ban_preference:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_ban_preference: %s\n" % e)
[inline-code-end]

---