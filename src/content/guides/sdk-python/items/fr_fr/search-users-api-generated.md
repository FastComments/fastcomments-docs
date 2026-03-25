## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| usernameStartsWith | string | query | Non |  |
| mentionGroupIds | array | query | Non |  |
| sso | string | query | Non |  |
| searchSection | string | query | Non |  |

## Réponse

Renvoie : [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple pour search_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users200_response import SearchUsers200Response
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est facultative et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (optionnel)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)
    search_section = 'search_section_example' # str |  (optionnel)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]