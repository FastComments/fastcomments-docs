## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| badgeId | string | query | Da |  |
| userId | string | query | Ne |  |
| commentId | string | query | Ne |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odziv

Vrača: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/award_user_badge_response.py)

## Primer

[inline-code-attrs-start title = 'Primer put_award_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.award_user_badge_response import AwardUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je izbirno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Odprite kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.ModerationApi(api_client)
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (izbirno)
    comment_id = 'comment_id_example' # str |  (izbirno)
    broadcast_id = 'broadcast_id_example' # str |  (izbirno)
    sso = 'sso_example' # str |  (izbirno)

    try:
        api_response = api_instance.put_award_badge(badge_id, user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of ModerationApi->put_award_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_award_badge: %s\n" % e)
[inline-code-end]