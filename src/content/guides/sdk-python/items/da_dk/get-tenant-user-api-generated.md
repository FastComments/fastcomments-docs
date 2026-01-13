## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Response

Returnerer: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_user200_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_tenant_user Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_user200_response import GetTenantUser200Response
from client.rest import ApiException
from pprint import pprint

# Angivelse af værten er valgfri og standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler på hver godkendelsesmetode er angivet nedenfor; brug det eksempel, der
# passer til dit brugstilfælde.

# Konfigurer API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant_user(tenant_id, id)
        print("The response of DefaultApi->get_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_user: %s\n" % e)
[inline-code-end]