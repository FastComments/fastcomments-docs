## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Respons

Returnerer: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_domain_config_response.py)

## Eksempel

[inline-code-attrs-start title = 'add_domain_config Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_domain_config_params import AddDomainConfigParams
from client.models.add_domain_config_response import AddDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standarden er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
# Klienten skal konfigurere autentificerings- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode vises nedenfor; brug det eksempel, der
# opfylder dit godkendelsesbehov.

# Konfigurer API-nøglegodkendelse: api_key
# Fjern kommentaren nedenfor for at sætte en præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    add_domain_config_params = client.AddDomainConfigParams() # AddDomainConfigParams | 

    try:
        api_response = api_instance.add_domain_config(tenant_id, add_domain_config_params)
        print("The response of DefaultApi->add_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_domain_config: %s\n" % e)
[inline-code-end]