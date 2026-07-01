## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## Réponse

Retourne : [`SearchUsersResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users_result.py)

## Exemple

[inline-code-attrs-start title = 'Exemple search_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import SearchUsersOptions
from client.models.search_users_result import SearchUsersResult
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est facultative et par défaut à https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (facultatif)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (facultatif)
    sso = 'sso_example' # str |  (facultatif)
    search_section = 'search_section_example' # str |  (facultatif)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, SearchUsersOptions(username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section))
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]