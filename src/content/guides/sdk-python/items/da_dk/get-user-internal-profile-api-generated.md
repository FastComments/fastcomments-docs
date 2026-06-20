---
## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_internal_profile_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_user_internal_profile Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_internal_profile_response import GetUserInternalProfileResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_user_internal_profile(comment_id=comment_id, sso=sso)
        print("The response of ModerationApi->get_user_internal_profile:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_user_internal_profile: %s\n" % e)
[inline-code-end]

---