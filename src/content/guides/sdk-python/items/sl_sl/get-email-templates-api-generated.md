## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_templates_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_email_templates'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_templates_response import GetEmailTemplatesResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Za seznam vseh podprtih konfiguracijskih parametrov glejte configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in pooblastil
# v skladu s varnostno politiko API strežnika.
# Spodaj so navedeni primeri za vsako metodo avtentikacije; uporabite primer, ki
# ustreza vašemu primeru uporabe.

# Konfigurirajte avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, če želite nastaviti predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_email_templates(tenant_id, skip=skip)
        print("The response of DefaultApi->get_email_templates:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_templates: %s\n" % e)
[inline-code-end]