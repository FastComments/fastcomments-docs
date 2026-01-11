## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Svar

Returnerer: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pages_api_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_pages Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pages_api_response import GetPagesAPIResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler på hver autentificeringsmetode er vist nedenfor; brug det eksempel der
# passer til dit autentificeringsbehov.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_pages(tenant_id)
        print("The response of DefaultApi->get_pages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pages: %s\n" % e)
[inline-code-end]