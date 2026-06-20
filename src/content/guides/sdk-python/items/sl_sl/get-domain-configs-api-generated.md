## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_configs_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_domain_configs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_configs_response import GetDomainConfigsResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre preverjanja pristnosti in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Spodaj so podani primeri za vsako metodo preverjanja pristnosti, uporabite primer, ki
# ustreza vašemu primeru uporabe preverjanja pristnosti.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API klienta
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_domain_configs(tenant_id)
        print("The response of DefaultApi->get_domain_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_configs: %s\n" % e)
[inline-code-end]