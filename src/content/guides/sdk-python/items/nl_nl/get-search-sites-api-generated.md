## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|------------|--------------|
| tenantId | string | query | Ja |  |
| value | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_site_search_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld get_search_sites'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetSearchSitesOptions
from client.models.moderation_site_search_response import ModerationSiteSearchResponse
from client.rest import ApiException
from pprint import pprint

# De host definiëren is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    value = 'value_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_search_sites(tenant_id, GetSearchSitesOptions(value=value, sso=sso))
        print("The response of ModerationApi->get_search_sites:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_sites: %s\n" % e)
[inline-code-end]