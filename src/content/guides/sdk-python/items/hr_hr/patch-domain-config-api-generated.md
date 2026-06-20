## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| domainToUpdate | string | path | Da |  |

## Response

Vraća: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_domain_config_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer patch_domain_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_domain_config_params import PatchDomainConfigParams
from client.models.patch_domain_config_response import PatchDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu sa sigurnosnom politikom API poslužitelja.
# Primjeri za svaku metodu autentikacije su prikazani u nastavku, upotrijebite primjer koji
# odgovara vašem slučaju upotrebe autentikacije.

# Konfigurirajte autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uklonite komentare ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
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