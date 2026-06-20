---
## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_manual_badges_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_manual_badges Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_manual_badges_response import GetTenantManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en heeft standaardwaarde https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.get_manual_badges(sso=sso)
        print("The response of ModerationApi->get_manual_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges: %s\n" % e)
[inline-code-end]

---