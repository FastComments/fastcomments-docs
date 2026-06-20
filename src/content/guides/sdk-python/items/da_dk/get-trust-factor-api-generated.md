## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_trust_factor Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og som standard bruges https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_trust_factor(user_id=user_id, sso=sso)
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]