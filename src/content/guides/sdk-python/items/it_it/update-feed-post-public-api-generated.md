## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| postId | string | path | Sì |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post_public200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di update_feed_post_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_public200_response import CreateFeedPostPublic200Response
from client.models.update_feed_post_params import UpdateFeedPostParams
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    update_feed_post_params = client.UpdateFeedPostParams() # UpdateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)

    try:
        api_response = api_instance.update_feed_post_public(tenant_id, post_id, update_feed_post_params, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->update_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_feed_post_public: %s\n" % e)
[inline-code-end]

---