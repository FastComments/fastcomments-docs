---
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nee |  |

## Response

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'delete_hash_tag Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.delete_hash_tag_request_body import DeleteHashTagRequestBody
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elk auth-mechanisme zijn hieronder voorbeelden opgenomen; gebruik het voorbeeld dat
# voldoet aan uw authenticatiegebruiksscenario.

# Configureer API-sleutel autorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal het commentaar hieronder weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (optioneel)
    delete_hash_tag_request_body = client.DeleteHashTagRequestBody() # DeleteHashTagRequestBody |  (optioneel)

    try:
        api_response = api_instance.delete_hash_tag(tag, tenant_id=tenant_id, delete_hash_tag_request_body=delete_hash_tag_request_body)
        print("The response of DefaultApi->delete_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_hash_tag: %s\n" % e)
[inline-code-end]

---