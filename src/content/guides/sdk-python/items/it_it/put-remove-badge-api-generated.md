## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Sì |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/remove_user_badge_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio put_remove_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.remove_user_badge_response import RemoveUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (opzionale)
    comment_id = 'comment_id_example' # str |  (opzionale)
    broadcast_id = 'broadcast_id_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.put_remove_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_remove_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_remove_badge: %s\n" % e)
[inline-code-end]