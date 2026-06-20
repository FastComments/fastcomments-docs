## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domainToUpdate | string | path | Ja |  |

## Antwoord

Retourneert: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_domain_config_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'patch_domain_config Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_domain_config_params import PatchDomainConfigParams
from client.models.patch_domain_config_response import PatchDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een overzicht van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elk authenticatiemethode zijn hieronder voorbeelden opgenomen; gebruik het voorbeeld dat
# past bij uw authenticatiegeval.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal onderstaande commentaar weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    patch_domain_config_params = client.PatchDomainConfigParams() # PatchDomainConfigParams | 

    try:
        api_response = api_instance.patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)
        print("The response of DefaultApi->patch_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_domain_config: %s\n" % e)
[inline-code-end]