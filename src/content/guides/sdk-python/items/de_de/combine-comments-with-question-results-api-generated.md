## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nein |  |
| questionIds | array | query | Nein |  |
| urlId | string | query | Nein |  |
| startDate | string | query | Nein |  |
| forceRecalculate | boolean | query | Nein |  |
| minValue | number | query | Nein |  |
| maxValue | number | query | Nein |  |
| limit | number | query | Nein |  |

## Response

Gibt zurück: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## Beispiel

[inline-code-attrs-start title = 'combine_comments_with_question_results Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben. Verwenden Sie das Beispiel, das
# Ihren Authentifizierungsanforderungen entspricht.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Entfernen Sie das Kommentarzeichen unten, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel festzulegen

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (optional)
    question_ids = ['question_ids_example'] # List[str] |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (optional)
    force_recalculate = True # bool |  (optional)
    min_value = 3.4 # float |  (optional)
    max_value = 3.4 # float |  (optional)
    limit = 3.4 # float |  (optional)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]