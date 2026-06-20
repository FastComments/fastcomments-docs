## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| startDate | string | query | Ne |  |
| questionId | string | query | Ne |  |
| questionIds | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Za seznam vseh podprtih konfiguracijskih parametrov si oglejte configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre avtentikacije in avtorizacije
# v skladu s varnostno politiko API strežnika.
# Spodaj so podani primeri za vsako metodo avtentikacije, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnjo vrstico za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (neobvezno)
    user_id = 'user_id_example' # str |  (neobvezno)
    start_date = 'start_date_example' # str |  (neobvezno)
    question_id = 'question_id_example' # str |  (neobvezno)
    question_ids = 'question_ids_example' # str |  (neobvezno)
    skip = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]