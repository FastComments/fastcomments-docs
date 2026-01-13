---
## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user200_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på create_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user200_response import CreateTenantUser200Response
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host; standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
# Klienten skal konfigurere autentificerings- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver autentificeringsmetode er angivet nedenfor, brug det
# eksempel, der passer til dit autentificeringsbehov.
# Konfigurer API-nøgleautorisation: api_key
# Fjern kommentar fra linjen nedenfor for at sætte et præfiks (fx Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_user_body = client.CreateTenantUserBody() # CreateTenantUserBody | 

    try:
        api_response = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
        print("The response of DefaultApi->create_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_user: %s\n" % e)
[inline-code-end]

---