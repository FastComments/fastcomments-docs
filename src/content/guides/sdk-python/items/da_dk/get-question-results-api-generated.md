## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nej |  |
| userId | string | query | Nej |  |
| startDate | string | query | Nej |  |
| questionId | string | query | Nej |  |
| questionIds | string | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_question_results Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetQuestionResultsOptions
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Definition af værten er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere godkendelses- og autoriseringsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver autentificeringsmetode er givet nedenfor, brug det eksempel som
# opfylder dit autentificeringsbrugssag.

# Konfigurer API-nøgle autorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at indstille præfiks (fx Bearer) for API-nøgle, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Indtast en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (valgfri)
    user_id = 'user_id_example' # str |  (valgfri)
    start_date = 'start_date_example' # str |  (valgfri)
    question_id = 'question_id_example' # str |  (valgfri)
    question_ids = 'question_ids_example' # str |  (valgfri)
    skip = 3.4 # float |  (valgfri)

    try:
        api_response = api_instance.get_question_results(tenant_id, GetQuestionResultsOptions(url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip))
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]