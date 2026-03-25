## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| state | number | query | Nee |  |
| skip | number | query | Nee |  |
| limit | number | query | Nee |  |

## Antwoord

Retourneert: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_tickets Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode worden hieronder gegeven; gebruik het
# voorbeeld dat past bij uw authenticatiesituatie.

# Configureer autorisatie met API-sleutel: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal de commentaarstreep hieronder weg om een voorvoegsel in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optioneel)
    state = 3.4 # float |  (optioneel)
    skip = 3.4 # float |  (optioneel)
    limit = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]