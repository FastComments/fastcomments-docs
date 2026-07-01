## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| broadcastId | string | query | Yes |  |
| sessionId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Esempio

[inline-code-attrs-start title = 'vote_comment Esempio'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import VoteCommentOptions
from client.models.vote_body_params import VoteBodyParams
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e predefinito a https://fastcomments.com
# Consulta configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    vote_body_params = client.VoteBodyParams() # VoteBodyParams | 
    session_id = 'session_id_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, VoteCommentOptions(session_id=session_id, sso=sso))
        print("The response of PublicApi->vote_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->vote_comment: %s\n" % e)
[inline-code-end]

---