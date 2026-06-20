## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| questionId | string | query | Ne |  |
| questionIds | array | query | Ne |  |
| urlId | string | query | Ne |  |
| startDate | string | query | Ne |  |
| forceRecalculate | boolean | query | Ne |  |
| minValue | number | query | Ne |  |
| maxValue | number | query | Ne |  |
| limit | number | query | Ne |  |

## Odgovor

Vraća: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## Primer

[inline-code-attrs-start title = 'Primer combine_comments_with_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevana vrednost je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da podesi parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Primeri za svaki metod autentifikacije su dati ispod; koristite primer
# koji odgovara vašem slučaju upotrebe.
# Konfigurišite autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite dole da biste podesili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (neobavezno)
    question_ids = ['question_ids_example'] # List[str] |  (neobavezno)
    url_id = 'url_id_example' # str |  (neobavezno)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (neobavezno)
    force_recalculate = True # bool |  (neobavezno)
    min_value = 3.4 # float |  (neobavezno)
    max_value = 3.4 # float |  (neobavezno)
    limit = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]