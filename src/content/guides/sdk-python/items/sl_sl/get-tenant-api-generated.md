## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vrne: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_tenant'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_response import GetTenantResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za overjanje in pooblastila
# v skladu z varnostno politiko API strežnika.
# Spodaj so prikazani primeri za vsak način avtentikacije, uporabite primer,
# ki ustreza vašemu primeru uporabe.

# Konfigurirajte avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, če je potrebno nastaviti predpono (npr. Bearer) za API ključ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant(tenant_id, id)
        print("The response of DefaultApi->get_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant: %s\n" % e)
[inline-code-end]