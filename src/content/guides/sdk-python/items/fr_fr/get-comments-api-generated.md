## Paramètres

| Name | Type | Location | Required | Description |
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

Renvoie : [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (optionnel)
    limit = 56 # int |  (optionnel)
    skip = 56 # int |  (optionnel)
    as_tree = True # bool |  (optionnel)
    skip_children = 56 # int |  (optionnel)
    limit_children = 56 # int |  (optionnel)
    max_tree_depth = 56 # int |  (optionnel)
    url_id = 'url_id_example' # str |  (optionnel)
    user_id = 'user_id_example' # str |  (optionnel)
    anon_user_id = 'anon_user_id_example' # str |  (optionnel)
    context_user_id = 'context_user_id_example' # str |  (optionnel)
    hash_tag = 'hash_tag_example' # str |  (optionnel)
    parent_id = 'parent_id_example' # str |  (optionnel)
    direction = client.SortDirections() # SortDirections |  (optionnel)
    from_date = 56 # int |  (optionnel)
    to_date = 56 # int |  (optionnel)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]