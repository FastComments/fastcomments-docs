---
## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_question_result'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.update_question_result_body import UpdateQuestionResultBody
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per default è https://fastcomments.com
# Consulta configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Gli esempi per ogni metodo di autenticazione sono forniti di seguito; usa quello
# che soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta qui sotto per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_question_result_body = client.UpdateQuestionResultBody() # UpdateQuestionResultBody | 

    try:
        api_response = api_instance.update_question_result(tenant_id, id, update_question_result_body)
        print("The response of DefaultApi->update_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_question_result: %s\n" % e)
[inline-code-end]

---