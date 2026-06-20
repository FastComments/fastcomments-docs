## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| postIds | array | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/feed_posts_stats_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_feed_posts_stats'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.feed_posts_stats_response import FeedPostsStatsResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est facultative et par défaut https://fastcomments.com
# Consultez configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] | 
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_feed_posts_stats(tenant_id, post_ids, sso=sso)
        print("The response of PublicApi->get_feed_posts_stats:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_feed_posts_stats: %s\n" % e)
[inline-code-end]