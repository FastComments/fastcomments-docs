## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_definitions_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_email_template_definitions'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_definitions_response import GetEmailTemplateDefinitionsResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Esempi per ogni metodo di autenticazione sono forniti di seguito; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta la riga sottostante per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_email_template_definitions(tenant_id)
        print("The response of DefaultApi->get_email_template_definitions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_definitions: %s\n" % e)
[inline-code-end]