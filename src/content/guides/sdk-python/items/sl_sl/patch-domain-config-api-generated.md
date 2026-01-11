## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Odgovor

Vrača: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_config200_response.py)

## Primer

[inline-code-attrs-start title = 'patch_domain_config Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_config200_response import GetDomainConfig200Response
from client.models.patch_domain_config_params import PatchDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za preverjanje pristnosti in avtorizacijo
# v skladu s politiko varnosti strežnika API.
# Spodaj so podani primeri za vsako metodo preverjanja, uporabite primer, ki
# ustreza vašemu primeru uporabe preverjanja.

# Konfiguracija avtentikacije z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Če je potrebno, odkomentirajte spodnje za nastavitev predpone (npr. Bearer) za API ključ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    patch_domain_config_params = client.PatchDomainConfigParams() # PatchDomainConfigParams | 

    try:
        api_response = api_instance.patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)
        print("The response of DefaultApi->patch_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_domain_config: %s\n" % e)
[inline-code-end]