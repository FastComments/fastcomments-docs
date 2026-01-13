## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| startDate | string | query | No |  |
| questionId | string | query | No |  |
| questionIds | string | query | No |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results200_response import GetQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Consultare configuration.py per una lista di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configurare l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta sotto per impostare il prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (opzionale)
    user_id = 'user_id_example' # str |  (opzionale)
    start_date = 'start_date_example' # str |  (opzionale)
    question_id = 'question_id_example' # str |  (opzionale)
    question_ids = 'question_ids_example' # str |  (opzionale)
    skip = 3.4 # float |  (opzionale)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]

---