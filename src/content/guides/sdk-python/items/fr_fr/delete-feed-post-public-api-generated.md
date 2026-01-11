## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| postId | string | path | Oui |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_feed_post_public200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_feed_post_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_feed_post_public200_response import DeleteFeedPostPublic200Response
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_id = 'post_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.delete_feed_post_public(tenant_id, post_id, broadcast_id=broadcast_id, sso=sso)
        print("The response of PublicApi->delete_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_feed_post_public: %s\n" % e)
[inline-code-end]