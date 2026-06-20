## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_result_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_question_result'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_result_response import GetQuestionResultResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione, usare l'esempio che
# soddisfa il vostro caso d'uso di autenticazione.

# Configurare l'autorizzazione con API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommentare la riga seguente per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_result(tenant_id, id)
        print("The response of DefaultApi->get_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_result: %s\n" % e)
[inline-code-end]

---