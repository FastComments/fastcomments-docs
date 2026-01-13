## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_question_configs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs200_response import GetQuestionConfigs200Response
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e predefinita a https://fastcomments.com
# Consultare configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# conformemente alla politica di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione; usare quello
# che soddisfa il proprio caso d'uso.

# Configurare l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommentare quanto segue per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]

---