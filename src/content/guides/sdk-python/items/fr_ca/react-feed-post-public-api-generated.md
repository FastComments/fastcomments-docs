## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| postId | string | path | Oui |  |
| isUndo | boolean | query | Non |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Response

Renvoie : [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/react_feed_post_public200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de react_feed_post_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.react_body_params import ReactBodyParams
from client.models.react_feed_post_public200_response import ReactFeedPostPublic200Response
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    react_body_params = client.ReactBodyParams() # ReactBodyParams | 
    is_undo = True # bool |  (optional)
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.react_feed_post_public(tenant_id, post_id, react_body_params, is_undo=is_undo, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->react_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->react_feed_post_public: %s\n" % e)
[inline-code-end]