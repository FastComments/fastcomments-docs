## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_from_comment_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_ban_users_from_comment Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_from_comment_response import GetBannedUsersFromCommentResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_ban_users_from_comment(comment_id, sso=sso)
        print("The response of ModerationApi->get_ban_users_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_ban_users_from_comment: %s\n" % e)
[inline-code-end]