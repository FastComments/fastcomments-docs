## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |

## Svar

Returnerer: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_reacts.py)

## Eksempel

[inline-code-attrs-start title = 'get_v2_page_reacts Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_reacts import GetV2PageReacts
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og er som standard https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en forekomst af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en forekomst af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_reacts(tenant_id, url_id)
        print("The response of PublicApi->get_v2_page_reacts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_reacts: %s\n" % e)
[inline-code-end]