requête  
tenantId  
afterId  

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| afterId | string | query | Non |  |
| limit | integer | query | Non |  |
| tags | array | query | Non |  |

## Réponse

Renvoie : [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_feed_posts'; type = 'python'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import client  
from client.api.default_api import GetFeedPostsOptions  
from client.models.get_feed_posts_response import GetFeedPostsResponse  
from client.rest import ApiException  
from pprint import pprint  

# La définition de l'hôte est optionnelle et la valeur par défaut est https://fastcomments.com  
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.  
configuration = client.Configuration(  
    host = "https://fastcomments.com"  
)  

# Le client doit configurer les paramètres d'authentification et d'autorisation  
# conformément à la politique de sécurité du serveur API.  
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui  
# répond à votre cas d'utilisation d'authentification.  

# Configurer l'autorisation de la clé API : api_key  
configuration.api_key['api_key'] = os.environ["API_KEY"]  

# Décommentez ci-dessous pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire  
# configuration.api_key_prefix['api_key'] = 'Bearer'  

# Entrer dans un contexte avec une instance du client API  
with client.ApiClient(configuration) as api_client:  
    # Créer une instance de la classe API  
    api_instance = client.DefaultApi(api_client)  
    tenant_id = 'tenant_id_example' # str |  
    after_id = 'after_id_example' # str |  (optional)  
    limit = 56 # int |  (optional)  
    tags = ['tags_example'] # List[str] |  (optional)  

    try:  
        api_response = api_instance.get_feed_posts(tenant_id, GetFeedPostsOptions(after_id=after_id, limit=limit, tags=tags))  
        print("The response of DefaultApi->get_feed_posts:\n")  
        pprint(api_response)  
    except Exception as e:  
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)  
[inline-code-end]  

---