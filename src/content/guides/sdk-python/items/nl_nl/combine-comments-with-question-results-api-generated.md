## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nee |  |
| questionIds | array | query | Nee |  |
| urlId | string | query | Nee |  |
| startDate | string | query | Nee |  |
| forceRecalculate | boolean | query | Nee |  |
| minValue | number | query | Nee |  |
| maxValue | number | query | Nee |  |
| limit | number | query | Nee |  |

## Respons

Retourneert: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_comments_with_question_results200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'combine_comments_with_question_results Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_comments_with_question_results200_response import CombineCommentsWithQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode worden hieronder gegeven, gebruik het voorbeeld dat
# past bij uw gebruikssituatie voor authenticatie.
# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal de commentaartekens hieronder weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (optioneel)
    question_ids = ['question_ids_example'] # List[str] |  (optioneel)
    url_id = 'url_id_example' # str |  (optioneel)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (optioneel)
    force_recalculate = True # bool |  (optioneel)
    min_value = 3.4 # float |  (optioneel)
    max_value = 3.4 # float |  (optioneel)
    limit = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]

---