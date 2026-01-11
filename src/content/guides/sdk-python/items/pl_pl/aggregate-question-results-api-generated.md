## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| questionId | string | query | Nie |  |
| questionIds | array | query | Nie |  |
| urlId | string | query | Nie |  |
| timeBucket | string | query | Nie |  |
| startDate | string | query | Nie |  |
| forceRecalculate | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results200_response import AggregateQuestionResults200Response
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby zobaczyć listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania, użyj przykładu który
# odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (opcjonalne)
    question_ids = ['question_ids_example'] # List[str] |  (opcjonalne)
    url_id = 'url_id_example' # str |  (opcjonalne)
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  (opcjonalne)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (opcjonalne)
    force_recalculate = True # bool |  (opcjonalne)

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate)
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]

---