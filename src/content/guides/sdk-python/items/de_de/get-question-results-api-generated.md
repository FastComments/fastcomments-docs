## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nein |  |
| userId | string | query | Nein |  |
| startDate | string | query | Nein |  |
| questionId | string | query | Nein |  |
| questionIds | string | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Returns: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetQuestionResultsOptions
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Das Definieren des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode werden unten bereitgestellt, verwenden Sie das Beispiel,
# das Ihren Authentifizierungsfall erfüllt.

# Konfigurieren Sie die API-Schlüssel-Authorisierung: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Kommentieren Sie unten aus, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls nötig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Betreten Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (optional)
    user_id = 'user_id_example' # str |  (optional)
    start_date = 'start_date_example' # str |  (optional)
    question_id = 'question_id_example' # str |  (optional)
    question_ids = 'question_ids_example' # str |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_question_results(tenant_id, GetQuestionResultsOptions(url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip))
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]