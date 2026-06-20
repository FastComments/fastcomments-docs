## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| trustFactor | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## Eksempel

[inline-code-attrs-start title = 'set_trust_factor Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# At definere host er valgfrit og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (valgfri)
    trust_factor = 'trust_factor_example' # str |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.set_trust_factor(user_id=user_id, trust_factor=trust_factor, sso=sso)
        print("The response of ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]