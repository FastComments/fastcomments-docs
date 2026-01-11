## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| commentId | string | path | Sì |  |
| voteId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| broadcastId | string | query | Sì |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_vote200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_comment_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_vote200_response import DeleteCommentVote200Response
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per default è https://fastcomments.com
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
    vote_id = 'vote_id_example' # str | 
    url_id = 'url_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->delete_comment_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_vote: %s\n" % e)
[inline-code-end]