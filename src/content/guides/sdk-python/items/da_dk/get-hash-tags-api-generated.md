## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | number | query | Nej |  |

## Svar

Returnerer: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_hash_tags_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_hash_tags Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_hash_tags_response import GetHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive vært, og standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er anført nedenfor, brug det eksempel der
# passer til dit brugsscenarie.

# Konfigurer API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at opsætte præfiks (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Opret en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_hash_tags(tenant_id, page=page)
        print("The response of DefaultApi->get_hash_tags:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_hash_tags: %s\n" % e)
[inline-code-end]