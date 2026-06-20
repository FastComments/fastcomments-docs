## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domainToUpdate | string | path | Ja |  |

## Respons

Returnerer: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_domain_config_response.py)

## Eksempel

[inline-code-attrs-start title = 'put_domain_config Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_domain_config_response import PutDomainConfigResponse
from client.models.update_domain_config_params import UpdateDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og er som standard https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere godkendelses- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er givet nedenfor, brug det eksempel der
# opfylder dit auth-brugstilfælde.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    update_domain_config_params = client.UpdateDomainConfigParams() # UpdateDomainConfigParams | 

    try:
        api_response = api_instance.put_domain_config(tenant_id, domain_to_update, update_domain_config_params)
        print("The response of DefaultApi->put_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_domain_config: %s\n" % e)
[inline-code-end]