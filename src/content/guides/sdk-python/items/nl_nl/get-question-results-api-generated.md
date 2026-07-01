## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| startDate | string | query | No |  |
| questionId | string | query | No |  |
| questionIds | string | query | No |  |
| skip | number | query | No |  |

## Reactie

Returns: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_question_results Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetQuestionResultsOptions
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke authenticatiemethode worden hieronder gegeven, gebruik het voorbeeld dat
# aansluit bij uw authenticatie‑use‑case.

# Configureer API‑sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Verwijder de commentaartekens hieronder om een prefix (bijv. Bearer) in te stellen voor de API‑sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API‑client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API‑klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (optioneel)
    user_id = 'user_id_example' # str |  (optioneel)
    start_date = 'start_date_example' # str |  (optioneel)
    question_id = 'question_id_example' # str |  (optioneel)
    question_ids = 'question_ids_example' # str |  (optioneel)
    skip = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_question_results(tenant_id, GetQuestionResultsOptions(url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip))
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]