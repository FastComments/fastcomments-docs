## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| page | integer | query | Non |  |
| limit | integer | query | Non |  |
| skip | integer | query | Non |  |
| asTree | boolean | query | Non |  |
| skipChildren | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| urlId | string | query | Non |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |
| contextUserId | string | query | Non |  |
| hashTag | string | query | Non |  |
| parentId | string | query | Non |  |
| direction | string | query | Non |  |
| fromDate | integer | query | Non |  |
| toDate | integer | query | Non |  |

## Réponse

Returns: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Exemple

[inline-code-attrs-start title = 'get_comments Exemple'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetCommentsOptions
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est facultative et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation en conformité avec la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui correspond à votre cas d'utilisation.

# Configurer l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (optional)
    limit = 56 # int |  (optional)
    skip = 56 # int |  (optional)
    as_tree = True # bool |  (optional)
    skip_children = 56 # int |  (optional)
    limit_children = 56 # int |  (optional)
    max_tree_depth = 56 # int |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)
    context_user_id = 'context_user_id_example' # str |  (optional)
    hash_tag = 'hash_tag_example' # str |  (optional)
    parent_id = 'parent_id_example' # str |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    from_date = 56 # int |  (optional)
    to_date = 56 # int |  (optional)

    try:
        api_response = api_instance.get_comments(tenant_id, GetCommentsOptions(page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date))
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]