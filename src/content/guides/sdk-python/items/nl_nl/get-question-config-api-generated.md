## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Response

Retourneert: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_config200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_question_config Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_config200_response import GetQuestionConfig200Response
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters instellen
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode zijn hieronder opgenomen, gebruik het voorbeeld dat
# past bij uw authenticatiescenario.

# Configureer API key authorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder de commentaarstreep weg om een prefix (bijv. Bearer) voor de API key in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_config(tenant_id, id)
        print("The response of DefaultApi->get_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_config: %s\n" % e)
[inline-code-end]