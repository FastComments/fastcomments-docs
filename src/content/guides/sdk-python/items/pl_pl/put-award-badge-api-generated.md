## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | zapytanie | Tak |  |
| badgeId | string | zapytanie | Tak |  |
| userId | string | zapytanie | Nie |  |
| commentId | string | zapytanie | Nie |  |
| broadcastId | string | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/award_user_badge_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład put_award_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PutAwardBadgeOptions
from client.models.award_user_badge_response import AwardUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Definiowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    badge_id = 'badge_id_example' # str | 
    user_id = 'user_id_example' # str |  (opcjonalny)
    comment_id = 'comment_id_example' # str |  (opcjonalny)
    broadcast_id = 'broadcast_id_example' # str |  (opcjonalny)
    sso = 'sso_example' # str |  (opcjonalny)

    try:
        api_response = api_instance.put_award_badge(tenant_id, badge_id, PutAwardBadgeOptions(user_id=user_id, comment_id=comment_id, broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->put_award_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_award_badge: %s\n" % e)
[inline-code-end]