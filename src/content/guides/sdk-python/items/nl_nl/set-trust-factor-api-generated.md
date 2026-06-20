## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| trustFactor | string | query | No |  |
| sso | string | query | No |  |

## Antwoord

Retourneert: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van set_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (optional)
    trust_factor = 'trust_factor_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.set_trust_factor(user_id=user_id, trust_factor=trust_factor, sso=sso)
        print("The response of ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]