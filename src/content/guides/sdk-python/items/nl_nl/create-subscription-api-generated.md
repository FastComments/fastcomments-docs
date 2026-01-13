## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Response

Retourneert: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_subscription_api_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'create_subscription Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_api_user_subscription_data import CreateAPIUserSubscriptionData
from client.models.create_subscription_api_response import CreateSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elk authenticatiemethode zijn hieronder voorbeelden opgenomen; gebruik het voorbeeld dat
# past bij uw authenticatiegeval.
# Configureer API-sleutelautorisatie: api_key
# Haal hieronder het commentaarteken weg om een voorvoegsel in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_user_subscription_data = client.CreateAPIUserSubscriptionData() # CreateAPIUserSubscriptionData | 

    try:
        api_response = api_instance.create_subscription(tenant_id, create_api_user_subscription_data)
        print("The response of DefaultApi->create_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_subscription: %s\n" % e)
[inline-code-end]