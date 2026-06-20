## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Sì |  |
| tenantId | string | query | No |  |

## Risposta

Restituisce: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di patch_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è facoltativa e predefinita a https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione; usa quello
# che soddisfa il tuo caso d'uso.

# Configura l'autorizzazione con API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta la riga sottostante per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (opzionale)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (opzionale)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]