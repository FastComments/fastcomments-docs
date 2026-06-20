## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Respons

Retourneert: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comment_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_comment Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comment_response import APIGetCommentResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode zijn hieronder opgenomen; gebruik het voorbeeld dat
# past bij uw authenticatiegeval.

# Configureer API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder de commentaarteken weg om de prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_comment(tenant_id, id)
        print("The response of DefaultApi->get_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comment: %s\n" % e)
[inline-code-end]