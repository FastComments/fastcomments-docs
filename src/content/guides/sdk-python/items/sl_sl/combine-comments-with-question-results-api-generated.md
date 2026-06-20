---
## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
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

Vrne: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## Primer

[inline-code-attrs-start title = 'combine_comments_with_question_results Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre overjanja in avtorizacije
# v skladu z varnostno politiko API strežnika.
# Spodaj so navedeni primeri za vsako metodo overjanja; uporabite primer,
# ki ustreza vašemu primeru uporabe.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Po potrebi odkomentirajte spodnje, da nastavite predpono (npr. Bearer) za API ključ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (neobvezno)
    question_ids = ['question_ids_example'] # List[str] |  (neobvezno)
    url_id = 'url_id_example' # str |  (neobvezno)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (neobvezno)
    force_recalculate = True # bool |  (neobvezno)
    min_value = 3.4 # float |  (neobvezno)
    max_value = 3.4 # float |  (neobvezno)
    limit = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]

---