## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## Antwoord

Retourneert: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_site_search_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_search_sites Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_site_search_response import ModerationSiteSearchResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.get_search_sites(value=value, sso=sso)
        print("The response of ModerationApi->get_search_sites:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_sites: %s\n" % e)
[inline-code-end]