## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_config_response.py)

## Primer

[inline-code-attrs-start title = 'Primer create_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_config_body import CreateQuestionConfigBody
from client.models.create_question_config_response import CreateQuestionConfigResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre za preverjanje pristnosti in avtorizacijo
# v skladu z varnostno politiko API strežnika.
# Spodaj so navedeni primeri za vsak način avtentikacije, uporabite tistega,
# ki ustreza vašemu primeru uporabe.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnje, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_config_body = client.CreateQuestionConfigBody() # CreateQuestionConfigBody | 

    try:
        api_response = api_instance.create_question_config(tenant_id, create_question_config_body)
        print("The response of DefaultApi->create_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_config: %s\n" % e)
[inline-code-end]