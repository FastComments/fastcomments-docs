## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| isFlagged | boolean | query | Yes |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio flag_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Apri un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    is_flagged = True # bool | 
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.flag_comment_public(tenant_id, comment_id, is_flagged, sso=sso)
        print("The response of PublicApi->flag_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->flag_comment_public: %s\n" % e)
[inline-code-end]