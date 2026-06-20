## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_get_comment_text_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.public_api_get_comment_text_response import PublicAPIGetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Consulta configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.get_comment_text(tenant_id, comment_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->get_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]