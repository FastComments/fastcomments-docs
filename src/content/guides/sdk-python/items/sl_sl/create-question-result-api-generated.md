## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_result_response.py)

## Primer

[inline-code-attrs-start title = 'Primer create_question_result'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_result_body import CreateQuestionResultBody
from client.models.create_question_result_response import CreateQuestionResultResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in je privzeto https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre preverjanja pristnosti in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Spodaj so podani primeri za vsako metodo avtentikacije, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_result_body = client.CreateQuestionResultBody() # CreateQuestionResultBody | 

    try:
        api_response = api_instance.create_question_result(tenant_id, create_question_result_body)
        print("The response of DefaultApi->create_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_result: %s\n" % e)
[inline-code-end]

---