## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domain | string | path | Yes |  |

## Svar

Returnerer: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_domain_config_response.py)

## Eksempel

[inline-code-attrs-start title = 'delete_domain_config Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_domain_config_response import DeleteDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Definering af host er valgfri og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentifikations- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er vist nedenfor; brug det eksempel, der
# passer til dit brugsscenarie.

# Konfigurer API-nøgleautorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentar fra nedenstående for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.delete_domain_config(tenant_id, domain)
        print("The response of DefaultApi->delete_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_domain_config: %s\n" % e)
[inline-code-end]