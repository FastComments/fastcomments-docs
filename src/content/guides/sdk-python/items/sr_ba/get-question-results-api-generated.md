## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| startDate | string | query | Ne |  |
| questionId | string | query | Ne |  |
| questionIds | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetQuestionResultsOptions
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumijeva https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
# Klijent mora konfigurisati parametre autentikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primjeri za svaki metod autentikacije su dati ispod, koristite primjer koji
# zadovoljava vaš slučaj upotrebe autentikacije.

# Konfigurišite autorizaciju API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite dole da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (opcionalno)
    user_id = 'user_id_example' # str |  (opcionalno)
    start_date = 'start_date_example' # str |  (opcionalno)
    question_id = 'question_id_example' # str |  (opcionalno)
    question_ids = 'question_ids_example' # str |  (opcionalno)
    skip = 3.4 # float |  (opcionalno)

    try:
        api_response = api_instance.get_question_results(tenant_id, GetQuestionResultsOptions(url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip))
        print("Odgovor DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak prilikom poziva DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]