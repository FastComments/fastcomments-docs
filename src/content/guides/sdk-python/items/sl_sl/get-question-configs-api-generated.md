## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | number | query | Ne |  |

## Odgovor

Vrača: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_question_configs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs_response import GetQuestionConfigsResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za avtentikacijo in avtorizacijo
# v skladu s politiko varnosti API strežnika.
# Primeri za vsako metodo avtentikacije so navedeni spodaj, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtorizacijo s ključem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Če je potrebno, odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]