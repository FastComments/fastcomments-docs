## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Reactie

Retourneert: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_create_user_badge_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'create_user_badge Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_create_user_badge_response import APICreateUserBadgeResponse
from client.models.create_user_badge_params import CreateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elke authenticatiemethode zijn hieronder voorbeelden opgenomen; gebruik het voorbeeld dat
# voldoet aan uw authenticatiebehoefte.

# Configureer API-sleutelautorisatie: api_key
# Haal hieronder de commentaartekens weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig

# Betreed een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_user_badge_params = client.CreateUserBadgeParams() # CreateUserBadgeParams | 

    try:
        api_response = api_instance.create_user_badge(tenant_id, create_user_badge_params)
        print("The response of DefaultApi->create_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_user_badge: %s\n" % e)
[inline-code-end]