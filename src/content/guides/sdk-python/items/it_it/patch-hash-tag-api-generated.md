## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tag | string | path | Sì |  |
| tenantId | string | query | No |  |

## Risposta

Restituisce: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_hash_tag200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio patch_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_hash_tag200_response import PatchHashTag200Response
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Consulta configuration.py per l'elenco di tutti i parametri di configurazione supportati.
# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.
# Configura l'autorizzazione tramite API key: api_key
# Decommenta la riga sottostante per impostare un prefisso (es. Bearer) per la API key, se necessario
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