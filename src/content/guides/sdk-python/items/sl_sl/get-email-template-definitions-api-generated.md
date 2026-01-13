## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_definitions200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_email_template_definitions'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_definitions200_response import GetEmailTemplateDefinitions200Response
from client.rest import ApiException
from pprint import pprint

# Določanje hosta je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za avtentikacijo in avtorizacijo
# v skladu s varnostno politiko API strežnika.
# Primeri za vsako metodo avtentikacije so podani spodaj, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Nastavite pooblastilo preko API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnjo vrstico, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_email_template_definitions(tenant_id)
        print("The response of DefaultApi->get_email_template_definitions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_definitions: %s\n" % e)
[inline-code-end]