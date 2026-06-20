---
## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| value | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_site_search_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_search_sites Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_site_search_response import ModerationSiteSearchResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og som standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_search_sites(value=value, sso=sso)
        print("The response of ModerationApi->get_search_sites:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_sites: %s\n" % e)
[inline-code-end]

---