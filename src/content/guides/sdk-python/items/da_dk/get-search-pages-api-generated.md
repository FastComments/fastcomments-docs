## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|----------|-------------|
| value | string | forespørgsel | Nej |  |
| sso | string | forespørgsel | Nej |  |

## Svar

Returnerer: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_page_search_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_search_pages Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_page_search_response import ModerationPageSearchResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_search_pages(value=value, sso=sso)
        print("The response of ModerationApi->get_search_pages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_pages: %s\n" % e)
[inline-code-end]

---