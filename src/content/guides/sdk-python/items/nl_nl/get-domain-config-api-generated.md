## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domain | string | path | Ja |  |

## Respons

Retourneert: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_config_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_domain_config Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_config_response import GetDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard https://fastcomments.com
# Bekijk configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode staan hieronder; gebruik het voorbeeld dat
# bij uw authenticatiegeval past.

# Stel API-sleutel autorisatie in: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal de commentaartekens hieronder weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.get_domain_config(tenant_id, domain)
        print("The response of DefaultApi->get_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_config: %s\n" % e)
[inline-code-end]