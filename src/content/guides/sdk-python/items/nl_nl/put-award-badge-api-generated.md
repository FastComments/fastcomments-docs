## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| badgeId | string | query | Ja |  |
| userId | string | query | Nee |  |
| commentId | string | query | Nee |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Geeft terug: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/award_user_badge_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'put_award_badge Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.award_user_badge_response import AwardUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (optioneel)
    comment_id = 'comment_id_example' # str |  (optioneel)
    broadcast_id = 'broadcast_id_example' # str |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.put_award_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_award_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_award_badge: %s\n" % e)
[inline-code-end]