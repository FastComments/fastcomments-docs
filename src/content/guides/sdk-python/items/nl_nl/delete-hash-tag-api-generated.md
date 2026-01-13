## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nee |  |

## Response

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'delete_hash_tag Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_hash_tag_request import DeleteHashTagRequest
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters
# configureren in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elk authenticatiemethode zijn hieronder voorbeelden opgenomen, gebruik het voorbeeld dat
# past bij uw gebruikssituatie.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder de commentaartekens weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (optioneel)
    delete_hash_tag_request = client.DeleteHashTagRequest() # DeleteHashTagRequest |  (optioneel)

    try:
        api_response = api_instance.delete_hash_tag(tag, tenant_id=tenant_id, delete_hash_tag_request=delete_hash_tag_request)
        print("The response of DefaultApi->delete_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_hash_tag: %s\n" % e)
[inline-code-end]