## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`BlockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/block_success.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di block_from_comment_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.block_success import BlockSuccess
from client.models.public_block_from_comment_params import PublicBlockFromCommentParams
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è facoltativa e di default è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    public_block_from_comment_params = client.PublicBlockFromCommentParams() # PublicBlockFromCommentParams | 
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso=sso)
        print("The response of PublicApi->block_from_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->block_from_comment_public: %s\n" % e)
[inline-code-end]