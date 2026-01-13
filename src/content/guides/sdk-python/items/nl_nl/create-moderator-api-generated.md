## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwoord

Retourneert: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_moderator200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'create_moderator Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_moderator200_response import CreateModerator200Response
from client.models.create_moderator_body import CreateModeratorBody
from client.rest import ApiException
from pprint import pprint

# Het opgeven van de host is optioneel en standaard is ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode worden hieronder gegeven; gebruik het voorbeeld dat
# voldoet aan jouw authenticatiegeval.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal de commentaartekens hieronder weg om een voorvoegsel in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_moderator_body = client.CreateModeratorBody() # CreateModeratorBody | 

    try:
        api_response = api_instance.create_moderator(tenant_id, create_moderator_body)
        print("The response of DefaultApi->create_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_moderator: %s\n" % e)
[inline-code-end]