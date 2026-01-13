## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nee |  |

## Response

Retourneert: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_templates200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_email_templates Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_templates200_response import GetEmailTemplates200Response
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst met alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode worden hieronder gegeven, gebruik het voorbeeld dat
# past bij uw authenticatiegebruik.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder de commentaar weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gebruik een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_email_templates(tenant_id, skip=skip)
        print("The response of DefaultApi->get_email_templates:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_templates: %s\n" % e)
[inline-code-end]