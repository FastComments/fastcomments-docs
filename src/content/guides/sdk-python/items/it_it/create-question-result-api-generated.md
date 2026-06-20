## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_result_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio create_question_result'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_result_body import CreateQuestionResultBody
from client.models.create_question_result_response import CreateQuestionResultResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Gli esempi per ogni metodo di autenticazione sono forniti di seguito; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta qui sotto per impostare il prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_result_body = client.CreateQuestionResultBody() # CreateQuestionResultBody | 

    try:
        api_response = api_instance.create_question_result(tenant_id, create_question_result_body)
        print("The response of DefaultApi->create_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_result: %s\n" % e)
[inline-code-end]