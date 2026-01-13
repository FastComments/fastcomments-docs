## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vraća: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_result200_response.py)

## Primjer

[inline-code-attrs-start title = 'create_question_result Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_result200_response import CreateQuestionResult200Response
from client.models.create_question_result_body import CreateQuestionResultBody
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurisati parametre autentifikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primjeri za svaku metodu autentifikacije su prikazani ispod — koristite primjer koji
# odgovara vašem slučaju upotrebe.

# Konfigurišite autorizaciju pomoću API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Napravite instancu API klase
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