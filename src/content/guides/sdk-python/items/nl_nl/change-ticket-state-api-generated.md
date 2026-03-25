## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Retourneert: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_ticket_state200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'change_ticket_state Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_ticket_state200_response import ChangeTicketState200Response
from client.models.change_ticket_state_body import ChangeTicketStateBody
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaardwaarde is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode worden hieronder gegeven, gebruik het voorbeeld dat
# past bij uw authenticatiegebruik.
# Configureer API-sleutelautorisatie: api_key
# Verwijder hieronder het commentaar om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    id = 'id_example' # str | 
    change_ticket_state_body = client.ChangeTicketStateBody() # ChangeTicketStateBody | 

    try:
        api_response = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
        print("The response of DefaultApi->change_ticket_state:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->change_ticket_state: %s\n" % e)
[inline-code-end]