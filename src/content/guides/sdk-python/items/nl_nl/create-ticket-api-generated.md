## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Ja |  |

## Antwoord

Retourneert: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_ticket200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'create_ticket Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_ticket200_response import CreateTicket200Response
from client.models.create_ticket_body import CreateTicketBody
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaardwaarde is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke authenticatiemethode worden hieronder gegeven; gebruik het voorbeeld dat
# past bij uw authenticatiescenario.

# Configureer autorisatie met API-sleutel: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder de commentaarteken weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    create_ticket_body = client.CreateTicketBody() # CreateTicketBody | 

    try:
        api_response = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
        print("The response of DefaultApi->create_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_ticket: %s\n" % e)
[inline-code-end]